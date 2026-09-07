// 23.7, 47.3 — GLUE: what a word fetched from a peer becomes here, and how
// two organs converge on one object.
//
//! A fetched word is replayed against this court's own tapes before it is
//! admitted to anything. A witness that RE-DERIVED the object converges by
//! `same_vertex` (M1) and never by comparing the bytes it is spelled with;
//! a have-list is a claim about an address, not a re-derivation.

use crate::cognition::*;
use crate::transducer::a21::Address;
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
        if let Some(reason) = crate::verdict::blind_replay_with(&spoken, &|a| self.resolve_form(a)) {
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
            .unwrap_or_else(crate::transducer::cech::Complex::new);
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
            peer.to_vec(),
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
                if let Some((voices, mu, anchor)) = self.treasury.release_award(&addr) {
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
                crate::transducer::a21::put_bytes(&mut buf, a);
            }
            if buf == d.root && members.len() as u32 == d.count {
                for addr in members {
                    if self.provisional.contains(&addr) {
                        eprintln!(
                            "  court peer holds {:02x}{:02x}… from {:02x}{:02x}…",
                            addr[0], addr[1], peer[0], peer[1]
                        );
                        self.home_install(&addr);
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

    pub(crate) fn home_install(&mut self, addr: &[u8]) {
        let held = self.word_at(addr).cloned().or_else(|| self.provisional.get(addr).cloned());
        if self.basis.get(addr).is_none() {
            if let Some(w) = held.as_ref() {
                if !(w.is_connection() && w.letters().is_none()) {
                    self.basis_push(w.clone(), [0; 32]);
                    self.fed_admit(addr, w.clone());
                }
            }
        }
        self.ruled_admit(addr);
        if let Some(w) = held {
            if w.is_rewrite() {
                if self.treasury.release_rewrite(addr).is_none() {
                    return;
                }
                self.retire_rewrite_inputs(&w);
            } else {
                let pay = self.verified_install_deflation(addr, &w);
                if pay.iter().all(|&x| x == 0) {
                    let _ = self.treasury.revoke_award(addr, false);
                    eprintln!(
                        "  court form installed {:02x}{:02x}… mint none (closes nothing)",
                        addr[0], addr[1]
                    );
                } else if let Some((voices, _, anchor)) = self.treasury.release_award(addr) {
                    self.treasury.mint_claim(&voices, pay, anchor);
                }
            }
        } else {
            let _ = self.treasury.revoke_award(addr, false);
            eprintln!(
                "  court form installed {:02x}{:02x}… mint none (closes nothing)",
                addr[0], addr[1]
            );
        }
        self.unlist(addr, false);
        self.provisional.remove(addr);
        self.court_upheld += 1;
        self.mark_persist();
        if let Some(w) = self.word_at(addr).cloned() {
            self.note_form_citations(addr, &w);
        }
        eprintln!("  court home installed {:02x}{:02x}… peer holds the stratum", addr[0], addr[1]);
    }

    pub(crate) fn note_form_citations(&mut self, addr: &[u8], w: &Element) {
        use crate::transducer::sentence::Vertex;
        let Some(f) = w.form() else {
            return;
        };
        for v in &f.vertices {
            let Vertex::Glued { address, .. } = v else {
                continue;
            };
            match self.cited_by.get_mut(address) {
                Some(bucket) => {
                    bucket.insert(addr, ());
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(addr, ());
                    self.cited_by.insert(address.clone(), bucket);
                }
            }
        }
        for (_, cert) in f.certificates.fiber(crate::network::nerve::ROOT) {
            let k = cert.key();
            match self.replay_memo.get_mut(addr) {
                Some(bucket) => {
                    bucket.insert(k, (true, self.chain.head_k()));
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(k, (true, self.chain.head_k()));
                    self.replay_memo.insert(addr, bucket);
                }
            }
            eprintln!(
                "  court pivot certified {:02x}{:02x}… loop {} axis {} {:02x}{:02x}→{:02x}{:02x}",
                addr[0],
                addr[1],
                cert.chord,
                cert.axis.label(),
                cert.before[0],
                cert.before[1],
                cert.after[0],
                cert.after[1]
            );
        }
    }

    pub(crate) fn redocket_citers_of(&mut self, cited: &[u8]) {
        let Some(bucket) = self.cited_by.remove(cited) else {
            return;
        };
        let citers: Vec<Vec<u8>> = bucket
            .fiber(crate::network::nerve::ROOT)
            .map(|(k, _)| k)
            .collect();
        for c in citers {
            if let Some(w) = self.word_at(&c).cloned() {
                self.provisional.insert(c, w);
            }
        }
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
        object: &crate::transducer::a21::Address,
        witness: [u8; 32],
        reached: Element,
    ) {
        if !self.provisional.contains(object) {
            return;
        }
        let (k, ht) = (self.k_step(), self.height());
        match self.converge_witness(object, witness, reached, k, ht) {
            crate::court::convergence::Converged::Witnessed { witness, .. } => {
                eprintln!(
                    "  court witness {:02x}{:02x}… co-signed {:02x}{:02x}… (replayed)",
                    witness[0], witness[1], object[0], object[1]
                );
                self.home_install(object);
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
    pub fn confirm_peer_holds_addrs(&mut self, have: &[crate::transducer::a21::Address], peer: [u8; 32]) {
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
                        self.home_install(h);
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
