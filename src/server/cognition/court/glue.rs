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

impl Kernel {
    pub fn glue_fetched(
        &mut self,
        spoken: Element,
        peer: [u8; 32],
        self_id: [u8; 32],
        cover: &mut crate::network::nerve::Cover,
    ) -> Glue {
        let addr = spoken.address();
        if self.heard.contains(&addr) {
            return Glue::Dialect;
        }
        if let Some(reason) = self.court_replay(&spoken) {
            eprintln!("  court glue slash {:02x}{:02x}… {reason}", addr[0], addr[1]);
            let inv = crate::transducer::stalk::invariant_of(&spoken);
            self.heard_mark(addr, &inv);
            return Glue::Slashed(reason);
        }
        let inv = crate::transducer::stalk::invariant_of(&spoken);
        let asg = element::assignment(spoken.sub());
        if spoken.is_sentence() {
            if let Some(f) = spoken.form() {
                // A pair form's Word vertices are the percept's own bodies, not
                // citations. Only Glued vertices name a held form that must resolve.
                let missing = f.vertices.iter().any(|v| match v {
                    crate::transducer::sentence::Vertex::Glued { address, .. } => {
                        address.clone() != addr && self.word_at(address).is_none()
                    }
                    _ => false,
                });
                if missing {
                    self.heard_mark(addr, &inv);
                    return Glue::Dialect;
                }
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
        let mut cx = self
            .complexes
            .remove(&inv.key())
            .unwrap_or_default();
        let vol0 = self.volume_of_cell(&inv, &[peer]);
        let peer_gamma = if spoken.is_translation() {
            spoken
                .letters()
                .map(|l| l.spelling.word.rotor())
                .unwrap_or_else(crate::transducer::alphabet::Rotor::identity)
        } else {
            crate::transducer::alphabet::Rotor::identity()
        };
        cx.set_section(
            peer,
            crate::transducer::cech::Section {
                addr: addr.clone(),
                gamma: peer_gamma,
            },
            vol0,
        );
        let self_rep = cx.sections.get(self_id.as_slice()).map(|s| s.addr.clone());
        let mut outcome = Glue::Admitted;
        let held_rep = self_rep.and_then(|a| self.word_at(&a).cloned());
        if let Some(rep) = &held_rep {
            match crate::transducer::cech::transition(rep, &spoken) {
                None => {}
                Some(g) if g.is_identity() => {}
                Some(g) if crate::transducer::stalk::placement_of(&g).is_some() => {
                    outcome = Glue::Attached;
                }
                Some(g) => {
                    outcome = Glue::Dialect;
                    if let Some(r) = crate::transducer::cech::rotor_of(&g) {
                        if let Some(v) = rotor_versor(&r) {
                            if let Some(ck) = r.class_key() {
                                self.push_cocycle(&inv, self_id, peer, v, ck);
                            }
                        }
                    }
                }
            }
        }
        let others: Vec<Vec<u8>> = verts.fiber(crate::network::nerve::ROOT).map(|(k, _)| k).collect();
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
            let vol_e = self.volume_of_cell(&inv, &[v.as_slice(), peer.as_slice()]);
            match crate::transducer::cech::transition(w, &spoken).and_then(|g| crate::transducer::cech::rotor_of(&g)) {
                Some(g) => cx.set_edge(v, peer.as_slice(), g, vol_e),
                None => {
                    if crate::transducer::cech::transition(w, &spoken).is_some() {
                        cx.set_remainder_edge(v, peer.as_slice());
                    }
                }
            }
        }
        cx.coboundary_on_link(&peer, |vs| self.volume_of_cell(&inv, vs));
        let mut filled = false;
        let mut class_cell: Option<(crate::transducer::alphabet::Rotor, u8)> = None;
        let mut congruent: Option<(crate::transducer::alphabet::Rotor, u32, Vec<Address>)> = None;
        let mut fill_gamma = crate::transducer::alphabet::Rotor::identity();
        let mut fill_addrs: Vec<Address> = Vec::new();
        let mut budget = self.physics.stroke();
        let spoken_rotor = spoken.letters().map(|l| l.spelling.word.rotor());
        // index: link of the fetched peer
        for key in cx.link_of(&peer) {
            let Some((d, volume, vertices)) = cx.cell(&key).map(|c| (c.degree(), c.volume, c.vertices.clone())) else {
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
                    cx.fill_by_gauge(&key, g)
                } else {
                    cx.fill(&key)
                }
            } else {
                cx.fill(&key)
            };
            match outcome_fill {
                crate::transducer::cech::Fill::Filled(gamma) => {
                    if volume.iter().any(|&x| x > 0) {
                        filled = true;
                        fill_gamma = gamma;
                        fill_addrs = vertices
                            .iter()
                            .filter_map(|v| cx.sections.get(v).map(|s| s.addr.clone()))
                            .collect();
                        eprintln!(
                            "  court cell filled degree {d} {:02x}{:02x}… volume {:?}",
                            key[0], key[1], volume
                        );
                    }
                }
                crate::transducer::cech::Fill::Congruent(g, ell) => {
                    let addrs: Vec<Address> = vertices
                        .iter()
                        .filter_map(|v| cx.sections.get(v).map(|s| s.addr.clone()))
                        .collect();
                    congruent = Some((g, ell, addrs));
                    eprintln!(
                        "  court cell congruent degree {d} {:02x}{:02x}…",
                        key[0], key[1]
                    );
                }
                crate::transducer::cech::Fill::Class(class) => {
                    class_cell = Some((class.clone(), d as u8));
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), Address::from(key.as_slice())));
                    if d >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class, d as u8));
                    }
                    eprintln!(
                        "  court cell class degree {d} {:02x}{:02x}…",
                        key[0], key[1]
                    );
                }
                crate::transducer::cech::Fill::Open => {
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), Address::from(key.as_slice())));
                }
                crate::transducer::cech::Fill::Already => {}
            }
        }
        self.complexes.insert(inv.key(), cx);
        if let Some(reason) = self.court_torsion(&spoken, &inv, self_id, peer) {
            self.heard_mark(addr, &inv);
            return Glue::Slashed(reason);
        }
        if filled {
            self.record_gamma(&fill_addrs, &fill_gamma);
            self.pay_cell_volume(&inv, &fill_addrs);
            if !fill_gamma.is_identity() {
                if let Some((a, b)) = edge_pair(&fill_addrs) {
                    self.mint_or_admit_translation(&fill_gamma, a, b, peer);
                }
            }
            outcome = Glue::Attached;
        }
        if let Some((_g, ell, addrs)) = congruent {
            self.file_torsion(&addrs, ell);
            self.pay_cell_volume(&inv, &addrs);
            self.pay_torse(&addrs);
            outcome = Glue::Attached;
        }
        if let Some((class, degree)) = class_cell {
            self.mark_class(&inv, class.clone(), degree);
            outcome = Glue::Dialect;
        }
        match outcome {
            Glue::Admitted => {
                if self.basis.get(&addr).is_none()
                    && !(spoken.is_connection() && spoken.letters().is_none())
                {
                    self.basis_push(spoken.clone(), peer);
                }
                if let Some(w) = self.word_mut(&addr) {
                    w.record_dislocation(asg, element::identity_versor(), element::FIBER_SPATIAL);
                    self.touch_at(&addr);
                }
                self.ruled_admit(&addr);
                self.fed_admit(&addr, spoken.clone());
                if let Some(crate::economy::escrow::Escrowed { voices, mu, anchor }) =
                    self.treasury.release_award(&addr)
                {
                    self.treasury.mint_claim(&voices, mu, anchor);
                }
                self.unlist(&addr, false);
                self.provisional.remove(&addr);
                self.court_upheld += 1;
                self.mark_persist();
            }
            Glue::Attached => {
                if let Some(rep) = held_rep {
                    if let Some(g) = crate::transducer::cech::transition(&rep, &spoken) {
                        if let Some(v) = crate::transducer::stalk::placement_of(&g) {
                            if let Some(w) = self.word_mut(&rep.address()) {
                                w.record_dislocation(asg, v, element::FIBER_SPATIAL);
                                self.touch_at(&rep.address());
                            }
                        }
                    }
                }
                self.ruled_admit(&addr);
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
        // index: the peer's digest charts; members from ruled fiber
        for d in remote {
            let members = self.ruled.fiber(&d.inv);
            let mut rows = members.clone();
            rows.sort();
            let mut buf = Vec::new();
            for a in &rows {
                crate::transducer::address::put_bytes(&mut buf, a);
            }
            if buf == d.root && members.len() as u32 == d.count {
                for addr in members {
                    if self.provisional.contains(&addr) {
                        eprintln!(
                            "  court peer holds {:02x}{:02x}… from {:02x}{:02x}…",
                            addr[0], addr[1], peer[0], peer[1]
                        );
                        self.home_install(&addr, peer);
                    }
                }
            }
            let ck = d.inv.key();
            let drop: Vec<Vec<u8>> = self
                .heard
                .fiber(crate::network::nerve::ROOT)
                .filter_map(|(addr, (chart, stored))| {
                    if chart.as_slice() == ck.as_bytes() && stored != &d.root {
                        Some(addr.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for addr in drop {
                self.heard.remove(&addr);
            }
        }
    }

    /// Deflation a form may mint: the descent speaking it measures on the
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
                    return self.measured_replay_descent(form, family);
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
    /// Nothing wrote one. `Complex::set_section` had exactly one production
    /// caller in the tree, `glue_fetched`'s `set_section(peer, …)`, always
    /// at the INCOMING peer's key. `self_rep = cx.sections.get(self_id)` was
    /// therefore always `None`, `held_rep` always `None`, and everything
    /// behind it dead: 13.9 step 3's difference on 0-cells, step 4's
    /// automorphism check, step 5's connecting image, 13.10's
    /// Consilient/Attached/Dialect table, and `push_cocycle` (M5). Every
    /// glue returned `Admitted` because there was no first vantage to
    /// compare a second against.
    pub(crate) fn section_own(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        addr: &crate::transducer::address::Address,
    ) {
        let me = self.identity;
        let vol = self.volume_of_cell(inv, &[me.as_slice()]);
        let mut cx = match self.complexes.remove(&inv.key()) {
            Some(c) => c,
            None => crate::transducer::cech::Complex::new(),
        };
        cx.set_section(
            me,
            crate::transducer::cech::Section {
                addr: addr.clone(),
                gamma: crate::transducer::alphabet::Rotor::identity(),
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
        // this witness whatever the two re-derivations turn out to be.
        self.note_witness_answered(object.as_bytes(), witness);
        let (k, ht) = (self.k_step(), self.height());
        match self.converge_witness(object, witness, reached, k, ht) {
            crate::court::convergence::Converged::Witnessed { witness, .. } => {
                eprintln!(
                    "  court witness {:02x}{:02x}… co-signed {:02x}{:02x}… (replayed)",
                    witness[0], witness[1], object[0], object[1]
                );
                self.home_install(object, witness);
            }
            crate::court::convergence::Converged::Inexact { a, b } => {
                eprintln!(
                    "  court form refused {:02x}{:02x}…: {} ({:02x}… vs {:02x}…)",
                    object[0],
                    object[1],
                    crate::court::verdict::REFUSE_MV_INEXACT,
                    a[0],
                    b[0]
                );
            }
            crate::court::convergence::Converged::AwaitingWitness => {}
        }
    }

    /// A peer's have-list naming a ruled provisional is the same evidence:
    /// the address is on their cover, so the home installs.
    pub fn confirm_peer_holds_addrs(&mut self, have: &[crate::transducer::address::Address], peer: [u8; 32]) {
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
                        eprintln!(
                            "  court witness {:02x}{:02x}… co-signed {:02x}{:02x}…",
                            witness[0], witness[1], h[0], h[1]
                        );
                        self.home_install(h, witness);
                    }
                    crate::court::convergence::Converged::Inexact { .. } => {
                        eprintln!(
                            "  court form refused {:02x}{:02x}…: {}",
                            h[0], h[1],
                            crate::court::verdict::REFUSE_MV_INEXACT
                        );
                    }
                    crate::court::convergence::Converged::AwaitingWitness => {
                        eprintln!(
                            "  court docketed {:02x}{:02x}…, awaiting witness",
                            h[0], h[1]
                        );
                    }
                }
            }
        }
    }

}
