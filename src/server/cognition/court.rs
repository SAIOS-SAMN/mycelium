// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The court organ: verdicts, the glue across hyphae, classes and
//! refinement, the ruled cover and what it serves.
//!
//! Split from cognition.rs under Stage 8's reading bound. A private
//! item is visible to its module AND its descendants, so these are the
//! same `impl Kernel` reaching the same private carriers: the split
//! moves code and changes nothing else.

use super::*;

impl Kernel {
    /// APPLY the court's verdict on a provisional word. The ruling is the
    /// closed 3-point vocabulary from the wire (never a product of booleans —
    /// bool×bool spans four states for a three-point space):
    /// RULING_UPHELD → release the escrowed award (finality books).
    /// RULING_DEMOTED → stays local dialect, the award evaporates.
    /// RULING_SLASHED → false speech: word excised, shares debited to the pool.
    /// Any other byte is refused — an unknown ruling rules nothing.
    pub fn apply_verdict(&mut self, addr: &[u8], ruling: u8) -> bool {
        use crate::anastome::dialogue::{RULING_DEMOTED, RULING_SLASHED, RULING_UPHELD};
        if !matches!(ruling, RULING_UPHELD | RULING_DEMOTED | RULING_SLASHED) {
            return false;
        }
        let spoken = self.provisional.get(addr).cloned();
        if self.provisional.remove(addr).is_none() {
            return false;
        }
        if ruling == RULING_UPHELD {
            self.ruled_admit(addr); // finalized here: it joins the ruled cover
            let spoken = spoken.or_else(|| self.word_at(addr).cloned());
            if let Some(w) = spoken {
                if w.is_rewrite() {
                    // Every voice the escrow names is paid; an escrow with
                    // no party is not released and the ruling does not land.
                    if self.treasury.release_rewrite(addr).is_none() {
                        return false;
                    }
                    self.retire_rewrite_inputs(&w);
                    self.fed_admit(addr, w);
                    self.unlist(addr, false);
                    self.mark_persist();
                    return true;
                }
                self.fed_admit(addr, w);
            }
            if let Some((voices, mu, anchor)) = self.treasury.release_award(addr) {
                self.treasury.mint_claim(&voices, mu, anchor);
            }
            self.unlist(addr, false);
            self.mark_persist();
            true
        } else {
            let slash = ruling == RULING_SLASHED;
            let ok = self.treasury.revoke_award(addr, slash);
            if slash {
                self.redocket_citers_of(addr);
                // A slashed word leaves the language and is admitted as a
                // cavity chart: the residual is a boundary, never garbage.
                if let Some(elem) = self.basis.get(addr).map(|w| (**w).clone()) {
                    self.basis_remove(addr);
                    let inv = crate::transducer::stalk::invariant_of(&elem);
                    self.listing_atlas.admit(&inv, addr);
                    self.write_field(
                        addr,
                        inv.clone(),
                        crate::transducer::stalk::Closure::Cavity,
                        elem.sub().to_vec(),
                        &elem.magnitude(),
                        &elem.magnitude(),
                    );
                    self.cavity_book.insert(&addr, elem.clone());
                    self.cavities.push(elem);
                }
                self.mark_persist();
            }
            ok
        }
    }

    /// Court organ: the docket is the field's projection — listings whose
    /// closure is not Flat and whose deflation is nonzero — plus unfilled
    /// triangles and tetrahedron classes. Fail → slash with a replay reason
    /// and relocate to the cavity atlas. Pass → a section on the ruled
    /// cover, escrow held. Fills and refinements share the stroke Budget.
    pub fn court_stroke(&mut self) {
        let mut budget = self.physics.stroke();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let from = self.open_cursor.clone();
        // index: budgeted keyed cursor over the open docket
        {
            let cur = self.open.cursor(&from, &mut budget);
            for (addr, _) in cur {
                keys.push(addr);
            }
        }
        if let Some(k) = keys.last() {
            self.open_cursor = k.clone();
        }
        for addr in keys {
            let spoken = self
                .word_at(&addr)
                .cloned()
                .or_else(|| self.book_elem(&addr).cloned());
            let Some(spoken) = spoken else {
                continue;
            };
            if let Some(reason) = crate::verdict::blind_replay_with(&spoken, &|a| self.resolve_form(a)) {
                eprintln!("  court slash {:02x}{:02x}… {reason}", addr[0], addr[1]);
                let _ = self.treasury.revoke_award(&addr, true);
                self.redocket_citers_of(&addr);
                self.relocate_cavity(&addr);
                self.provisional.remove(&addr);
                self.court_slashed += 1;
                continue;
            }
            self.ruled_admit(&addr);
            eprintln!("  court section {:02x}{:02x}… ruled cover, escrow held", addr[0], addr[1]);
        }
        self.lapse_docket();
        self.relocate_t_exhausted();
        self.stroke_fills(&mut budget);
    }

    /// 47.5 / 5.5: a listing whose standing height is an aperture of
    /// admitted receipts behind the head releases its draw. Escrow returns;
    /// nothing is slashed. Λ is aperture_cycle (the measured reserve ratio).
    fn lapse_docket(&mut self) {
        let now = self.height();
        let lambda = crate::physics::aperture_cycle() as u64;
        let mut lapse: Vec<crate::transducer::a21::Address> = Vec::new();
        for (addr, _) in self.pending.fiber(crate::network::nerve::ROOT) {
            let Some(&h0) = self.standing_h.get(&addr) else {
                continue;
            };
            if now.saturating_sub(h0) >= lambda {
                lapse.push(Address::from(addr));
            }
        }
        for addr in lapse {
            let n = now.saturating_sub(self.standing_h.get(&addr).copied().unwrap_or(now));
            eprintln!(
                "  court citation lapsed {:02x}{:02x}… after {n} receipts",
                addr[0], addr[1]
            );
            let _ = self.treasury.revoke_award(&addr, false);
            self.unlist(&addr, false);
        }
    }

    fn stroke_fills(&mut self, budget: &mut crate::physics::Budget) {
        if budget.remaining() == 0 {
            return;
        }
        let mut pending: Vec<(crate::transducer::stalk::Invariant, crate::transducer::a21::Address)> = Vec::new();
        for (_, v) in self.unfilled.cursor(crate::network::nerve::ROOT, budget) {
            pending.push(v.clone());
        }
        for (inv, sim_key) in pending {
            let dkey = chart_simplex_key(&inv, &sim_key);
            self.unfilled.remove(&dkey);
            let filled = {
                let Some(cx) = self.complexes.get_mut(&inv.key()) else {
                    continue;
                };
                let addrs: Vec<Address> = cx
                    .sections
                    .fiber(crate::network::nerve::ROOT)
                    .map(|(_, s)| s.addr.clone())
                    .collect();
                (cx.fill(&sim_key), addrs)
            };
            match filled {
                (crate::transducer::cech::Fill::Filled(gamma), addrs) => {
                    eprintln!(
                        "  court cell filled {:02x}{:02x}… γ recorded",
                        sim_key[0], sim_key[1]
                    );
                    self.record_gamma(&addrs, &gamma);
                    self.pay_cell_volume(&inv, &addrs);
                    if !gamma.is_identity() {
                        if let Some((a, b)) = edge_pair(&addrs) {
                            self.mint_or_admit_translation(&gamma, a.clone(), b.clone(), [0; 32]);
                            let key = Self::cocycle_key(&inv, &a, &b);
                            if let Some((v, tu, _, _, _)) = self.cocycle_at.remove(&key) {
                                self.dialect_chain.push((key.clone(), v, tu));
                            }
                            if let Some(m) = self.cocycle_by_chart.get_mut(&inv.key()) {
                                m.remove(&key);
                            }
                        }
                    }
                }
                (crate::transducer::cech::Fill::Congruent(_g, _ell), addrs) => {
                    eprintln!(
                        "  court cell congruent {:02x}{:02x}… torsion filed",
                        sim_key[0], sim_key[1]
                    );
                    self.file_torsion(&addrs, _ell);
                    self.pay_cell_volume(&inv, &addrs);
                    self.pay_torse(&addrs);
                }
                (crate::transducer::cech::Fill::Class(class), _) => {
                    eprintln!(
                        "  court cell class {:02x}{:02x}…",
                        sim_key[0], sim_key[1]
                    );
                    let degree = self
                        .complexes
                        .get(&inv.key())
                        .and_then(|c| c.class(&sim_key).map(|(_, d)| d as u8))
                        .unwrap_or(2);
                    self.mark_class(&inv, class.clone(), degree);
                    if degree >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class, degree));
                    }
                }
                (crate::transducer::cech::Fill::Open, _) | (crate::transducer::cech::Fill::Already, _) => {}
            }
        }
        let mut refine: Vec<(
            crate::transducer::stalk::Invariant,
            crate::transducer::alphabet::Rotor,
            u8,
        )> = Vec::new();
        for (_, v) in self.pending_refine.cursor(crate::network::nerve::ROOT, budget) {
            refine.push(v.clone());
        }
        for (inv, class, degree) in refine {
            let key = inv.key();
            self.pending_refine.remove(&key);
            eprintln!(
                "  court class degree {degree} {:02x}{:02x}… chart refined",
                key[0], key[1]
            );
            self.refine_chart(&inv, class, degree, None);
        }
    }

    pub(super) fn listings_on<A: AsRef<[u8]>>(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        verts: &[A],
    ) -> crate::network::nerve::Keyed<()> {
        let Some(on_chart) = self.chart_listings.get(&inv.key()) else {
            return crate::network::nerve::Keyed::new();
        };
        let mut acc: Option<crate::network::nerve::Keyed<()>> = None;
        for v in verts {
            let Some(held) = self.spoken_by.get(v.as_ref()) else {
                return crate::network::nerve::Keyed::new();
            };
            let here = held.meet(on_chart).meet(&self.open);
            acc = Some(match acc {
                None => here,
                Some(prev) => prev.meet(&here),
            });
        }
        acc.unwrap_or_default()
    }

    fn volume_of_cell<A: AsRef<[u8]>>(&self, inv: &crate::transducer::stalk::Invariant, verts: &[A]) -> [u64; 4] {
        let held = self.listings_on(inv, verts);
        let mut chain = Vec::new();
        for (a, _) in held.fiber(crate::network::nerve::ROOT) {
            if let Some(f) = self.field.get(&a) {
                chain.extend(union_remainders(&f.entries));
            }
        }
        element::measure_lanes(&chain)
    }

    fn pay_cell_volume<A: AsRef<[u8]>>(&mut self, inv: &crate::transducer::stalk::Invariant, addrs: &[A]) {
        let held = self.listings_on(inv, addrs);
        let vol = self.volume_of_cell(inv, addrs);
        // Every vertex of the cell is a party to what the cell closed. The
        // volume is one quantity for all of them, so it splits per lane as
        // a consilience award does; paying `addrs.first()` paid whichever
        // member the cell happened to list first (rule 15).
        let parties: Vec<[u8; 32]> = addrs
            .iter()
            .filter_map(|a| <[u8; 32]>::try_from(a.as_ref()).ok())
            .collect();
        if parties.is_empty() {
            return;
        }
        let n = parties.len() as u64;
        let share: [u64; 4] =
            std::array::from_fn(|l| if vol[l] == 0 { 0 } else { (vol[l] / n).max(1) });
        let vol = share;
        let anchor = self.anchor();
        for (a, _) in held.fiber(crate::network::nerve::ROOT) {
            for v in &parties {
                let _ = self.treasury.draw_escrow(&a, *v, vol, anchor);
            }
        }
    }

    /// Each member of the cell is paid the torsion IT measured. Reading the
    /// magnitude off `addrs.first()` made the value of a torsion depend on
    /// which member the cell happened to list first, and paid it to that
    /// member alone; a member with no measured tension is unmeasured and is
    /// paid nothing, never paid another member's reading (rule 11, 15).
    fn pay_torse<A: AsRef<[u8]>>(&mut self, addrs: &[A]) {
        let anchor = self.anchor();
        let parties: Vec<([u8; 32], [u64; 4])> = addrs
            .iter()
            .filter_map(|a| {
                let who = <[u8; 32]>::try_from(a.as_ref()).ok()?;
                let mag = self.tension.get(a.as_ref()).copied()?;
                Some((who, mag))
            })
            .collect();
        for (who, mag) in parties {
            let _ = self.treasury.settle_torse(who, mag, anchor);
        }
    }

    fn file_torsion<A: AsRef<[u8]>>(&mut self, addrs: &[A], ell: u32) {
        let Some(rep) = addrs.first() else {
            return;
        };
        let addr = rep.as_ref();
        let Some(asg) = self.basis.get(addr).map(|w| element::assignment(w.sub())) else {
            return;
        };
        if let Some(wm) = self.word_mut(addr) {
            wm.record_torsion(asg, ell, element::FIBER_SPATIAL);
        }
        self.touch_at(addr);
    }

    /// A gauge from Extended between two hyphas is a translation: minted
    /// through from_connection with the identity gauge map, tagged, entered
    /// on the docket, and admitted to the ruled cover on second replay.
    fn mint_or_admit_translation(
        &mut self,
        gamma: &crate::transducer::alphabet::Rotor,
        a: crate::transducer::a21::Address,
        b: crate::transducer::a21::Address,
        ratifier: [u8; 32],
    ) {
        let Some(t) = element::from_translation(gamma, &a, &b) else {
            return;
        };
        let addr = t.address();
        if self.ruled_contains(&addr) {
            return;
        }
        if self.provisional.contains(&addr) {
            self.ruled_admit(&addr);
            self.fed_admit(&addr, t);
            self.provisional.remove(&addr);
            self.court_upheld += 1;
            eprintln!(
                "  court translation admitted {:02x}{:02x}… second replay",
                addr[0], addr[1]
            );
            return;
        }
        self.docket_connection(&t, ratifier);
        let payload = crate::transducer::rewrite::RewritePayload::Translation {
            a: a.clone(),
            b: b.clone(),
            gauge: element::identity_versor(),
        };
        let _ = self.mint_rewrite(
            crate::transducer::rewrite::RewriteKind::Translation,
            &[],
            &t,
            payload,
            ratifier,
        );
        eprintln!(
            "  court translation docked {:02x}{:02x}…",
            addr[0], addr[1]
        );
    }

    /// Glue a word fetched from a peer whose cover meets ours on this chart.
    /// Vertices are covering hyphas. Degree 0 is the peer's section; degree 1
    /// edges to held sections; then coboundary_on_link climbs while volume
    /// persists.
    /// Match spoken torsion cycles against the chart's Congruent cells.
    /// Same class key and ℓ is consilience; same key different ℓ is
    /// [`crate::verdict::REFUSE_NOT_NATURAL`]; a new finite key files.
    /// A v2 form reads `Form.torsion` (keyed by the filed loop's class);
    /// a residual word still reads the element's cycles.
    pub(super) fn court_torsion(
        &mut self,
        spoken: &Element,
        inv: &crate::transducer::stalk::Invariant,
        self_id: [u8; 32],
        peer: [u8; 32],
    ) -> Option<&'static str> {
        let cycles = Self::spoken_torsion_cycles(spoken);
        if cycles.is_empty() {
            return None;
        }
        for (class, ell, rep) in cycles {
            let held = self
                .complexes
                .get(&inv.key())
                .and_then(|cx| cx.congruent(&class).map(|(_, ell2)| *ell2));
            match held {
                Some(ell2) if ell2 != ell => {
                    eprintln!(
                        "  court torsion refused {:02x}{:02x}… ℓ {ell}",
                        class[0], class[1]
                    );
                    return Some(crate::verdict::REFUSE_NOT_NATURAL);
                }
                Some(_) => {
                    eprintln!(
                        "  court torsion consilient {:02x}{:02x}… ℓ {ell}",
                        class[0], class[1]
                    );
                }
                None => {
                    if let Some(rep) = rep {
                        if let Some(cx) = self.complexes.get_mut(&inv.key()) {
                            cx.file_congruent(self_id, peer, rep, ell);
                        }
                    }
                    eprintln!(
                        "  court torsion filed {:02x}{:02x}… ℓ {ell}",
                        class[0], class[1]
                    );
                }
            }
        }
        None
    }

    /// Torsion cycles a spoken word names: class key, period, representative
    /// rotor of the filed loop. Residual germs keep the element's section.
    fn spoken_torsion_cycles(
        spoken: &Element,
    ) -> Vec<(Address, u32, Option<crate::transducer::alphabet::Rotor>)> {
        use crate::transducer::sentence::torsion_loop_rotor;
        if let Some(f) = spoken.form() {
            if !f.torsion.is_empty() {
                return f
                    .torsion
                    .fiber(crate::network::nerve::ROOT)
                            .map(|(k, (_, ell, _))| {
                        let rotor = torsion_loop_rotor(f, &k);
                        (Address::from(k), *ell, rotor)
                    })
                    .collect();
            }
        }
        spoken
            .torsion()
            .iter()
            .filter_map(|(_, ell, _)| {
                let r = spoken.letters()?.spelling.word.rotor();
                Some((r.class_key()?, *ell, Some(r)))
            })
            .collect()
    }

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

    fn record_gamma<A: AsRef<[u8]>>(&mut self, addrs: &[A], gamma: &crate::transducer::alphabet::Rotor) {
        let Some(v) = rotor_versor(gamma) else {
            return;
        };
        for addr in addrs {
            let key = addr.as_ref();
            let Some(asg) = self.basis.get(key).map(|w| element::assignment(w.sub())) else {
                continue;
            };
            if let Some(w) = self.word_mut(key) {
                w.record_dislocation(asg, v.clone(), element::FIBER_SPATIAL);
            }
            self.touch_at(key);
        }
    }

    fn mark_class(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        class: crate::transducer::alphabet::Rotor,
        degree: u8,
    ) {
        let members: Vec<Vec<u8>> = self.listing_atlas.fiber(inv);
        for addr in members {
            let Some(mag) = self.tension.get(&addr).copied() else {
                continue;
            };
            self.write_field(
                &addr,
                inv.clone(),
                crate::transducer::stalk::Closure::Class(class.clone(), degree),
                Vec::new(),
                &mag,
                &mag,
            );
        }
    }

    fn refine_chart(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        class: crate::transducer::alphabet::Rotor,
        _degree: u8,
        mut cover: Option<&mut crate::network::nerve::Cover>,
    ) {
        let axes = class.class_axes().unwrap_or_else(|| vec![0]);
        let cx = self.complexes.get(&inv.key()).cloned();
        let coord_of = |addr: &[u8]| -> Vec<u64> {
            if let Some(c) = &cx {
                for (_, sec) in c.sections.fiber(crate::network::nerve::ROOT) {
                    if sec.addr.as_bytes() == addr && !sec.gamma.is_identity() {
                        return axes.clone();
                    }
                }
            }
            vec![0]
        };
        let before: Vec<Vec<u8>> = self.listing_atlas.fiber(inv);
        let before_mag: Vec<(Vec<u8>, [u64; 4])> = before
            .iter()
            .filter_map(|a| self.tension.get(a).copied().map(|t| (a.clone(), t)))
            .collect();
        self.ruled.split_stratum(inv, coord_of);
        std::sync::Arc::make_mut(&mut self.stalks).split_stratum(inv, coord_of);
        self.listing_atlas.split_stratum(inv, coord_of);
        if let Some(cover) = cover.as_mut() {
            for lane in 0..4u8 {
                let hyphas = cover.fiber(lane, inv);
                for h in hyphas {
                    let side = cx
                        .as_ref()
                        .and_then(|c| c.sections.get(&h))
                        .map(|sec| {
                            if sec.gamma.is_identity() {
                                vec![0]
                            } else {
                                axes.clone()
                            }
                        })
                        .unwrap_or_else(|| vec![0]);
                    let new = crate::transducer::stalk::Stalks::refine_axes(inv, &side);
                    cover.rehome(lane, inv, &new, h);
                }
            }
        }
        for (addr, mag) in before_mag {
            let Some(after) = self.tension.get(&addr).copied() else {
                continue; // Unmeasured: no tension is not zero deflation
            };
            if let Some(d) = crate::economy::split_deflation(&mag, &after) {
                self.deflations.insert(addr, d);
            }
        }
        if let Some(c) = self.complexes.remove(&inv.key()) {
            let a = crate::transducer::stalk::Stalks::refine_axes(inv, &[0]);
            let b = crate::transducer::stalk::Stalks::refine_axes(inv, &axes);
            self.rekey_cocycles(inv, &a, &b);
            self.complexes.insert(a.key(), c.clone());
            self.complexes.insert(b.key(), c);
        }
        self.basis_gen += 1;
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
    fn verified_install_deflation(&self, addr: &[u8], form: &Element) -> [u64; 4] {
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

    fn home_install(&mut self, addr: &[u8]) {
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

    fn note_form_citations(&mut self, addr: &[u8], w: &Element) {
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
                    bucket.insert(&addr, ());
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(&addr, ());
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
                    self.replay_memo.insert(&addr, bucket);
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

    fn redocket_citers_of(&mut self, cited: &[u8]) {
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

    /// A peer's have-list naming a ruled provisional is the same evidence:
    /// the address is on their cover, so the home installs.
    pub fn confirm_peer_holds_addrs(&mut self, have: &[crate::transducer::a21::Address], peer: [u8; 32]) {
        // index: the peer's have-list
        for h in have {
            if self.provisional.contains(h) && self.ruled_contains(h) {
                eprintln!(
                    "  court peer holds {:02x}{:02x}… from {:02x}{:02x}…",
                    h[0], h[1], peer[0], peer[1]
                );
                self.home_install(h);
            }
        }
    }

    /// Charts this hypha covers: Twisted or Cavity listings, per occupied lane.
    /// Never the whole pending book — that clone was the last deploy's leak.
    pub fn own_cover_charts(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        // index: this hypha's own cover trie (signed digest payload)
        self.own_cover.all_charts()
    }

    pub fn complexes(&self) -> &crate::network::nerve::Keyed<crate::transducer::cech::Complex> {
        &self.complexes
    }

    /// Ideology boundary on a chart: classes at every degree the mesh holds
    /// there, including coarser prefixes, so a mind can be taught the frame
    /// its bubble sits on.
    pub fn classes_on(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<(crate::transducer::alphabet::Rotor, u8)> {
        let mut out = Vec::new();
        for p in inv.prefixes() {
            if let Some(cx) = self.complexes.get(&p.key()) {
                for (r, d) in cx.classes() {
                    out.push((r, d as u8));
                }
            }
        }
        out
    }

    pub fn cochain_bytes(&self) -> usize {
        let mut n = 0usize;
        for (_, c) in self.complexes.fiber(crate::network::nerve::ROOT) {
            n += c.byte_len();
        }
        n
    }

    /// Class records on `inv` (and prefixes), or on every chart when `inv` is empty.
    pub fn class_records(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<crate::anastome::dialogue::ClassPage> {
        let mut out = Vec::new();
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let push = |out: &mut Vec<crate::anastome::dialogue::ClassPage>,
                    seen: &mut crate::network::nerve::Keyed<()>,
                    r: &crate::transducer::alphabet::Rotor,
                    order: u32| {
            let Some(k) = r.class_key() else {
                return;
            };
            if seen.contains(&k) {
                return;
            }
            seen.insert(k.clone(), ());
            let mut rotor = Vec::new();
            r.to_bytes(&mut rotor);
            out.push(crate::anastome::dialogue::ClassPage {
                key: k,
                order,
                rotor,
            });
        };
        if inv.0.is_empty() {
            for (_, cx) in self.complexes.fiber(crate::network::nerve::ROOT) {
                for layer in &cx.degrees {
                    for (_, cell) in layer.fiber(crate::network::nerve::ROOT) {
                        match &cell.fill {
                            crate::transducer::cech::Fill::Congruent(r, ell) => {
                                push(&mut out, &mut seen, r, *ell)
                            }
                            crate::transducer::cech::Fill::Class(r) => {
                                push(&mut out, &mut seen, r, 0)
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            for p in inv.prefixes() {
                if let Some(cx) = self.complexes.get(&p.key()) {
                    for layer in &cx.degrees {
                        for (_, cell) in layer.fiber(crate::network::nerve::ROOT) {
                            match &cell.fill {
                                crate::transducer::cech::Fill::Congruent(r, ell) => {
                                    push(&mut out, &mut seen, r, *ell)
                                }
                                crate::transducer::cech::Fill::Class(r) => {
                                    push(&mut out, &mut seen, r, 0)
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        out
    }

    pub fn serve_ruled(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        have: &[crate::transducer::a21::Address],
        budget: usize,
    ) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        let mut used = 0usize;
        let mut held: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for h in have {
            held.insert(h, ());
        }
        // An empty mind is seeded from the root: coarsest ruled words first.
        let addrs: Vec<Vec<u8>> = if inv.0.is_empty() {
            self.syllabus_seed()
        } else {
            self.ruled.fiber(inv)
        };
        for addr in addrs {
            if held.contains(&addr) {
                continue;
            }
            let Some(w) = self.word_at(&addr) else { continue };
            if w.is_connection() && w.letters().is_none() {
                continue; // letterless history is never taught
            }
            let bytes = w.to_bytes();
            if used + bytes.len() + 4 > budget {
                break;
            }
            used += bytes.len() + 4;
            out.push(bytes);
        }
        out
    }

    pub fn ruled_digests(&self) -> Vec<crate::transducer::sheaf::StratumDigest> {
        crate::transducer::sheaf::digests(&self.ruled)
    }

    pub fn ruled_have(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        self.ruled.fiber(inv).to_vec()
    }

    /// Ruled addresses, coarsest chart first — the syllabus seed of an
    /// empty atlas.
    pub fn syllabus_seed(&self) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        for (_, members) in self.ruled.strata() {
            out.extend(members);
        }
        out
    }

    pub fn fetch_have(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        let mut h: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.ruled.fiber(inv) {
            h.insert(a, ());
        }
        for (a, _) in self.heard.fiber(crate::network::nerve::ROOT) {
            h.insert(a, ());
        }
        h.fiber(crate::network::nerve::ROOT).map(|(k, _)| k).collect()
    }

    pub fn bind_listing_task(&mut self, listing: impl AsRef<[u8]>, task: impl AsRef<[u8]>) {
        let listing = Address::from_bytes(listing);
        let task = Address::from_bytes(task);
        self.listing_task.insert(&listing, task.clone());
        match self.task_listings.get_mut(task.as_bytes()) {
            Some(v) => {
                if !v.iter().any(|a| a == &listing) {
                    v.push(listing);
                }
            }
            None => {
                self.task_listings.insert(&task, vec![listing]);
            }
        }
    }

    /// The trains a rule must speak to mint at install. Empty trains are ignored.
    pub fn bind_task_family(&mut self, task: impl AsRef<[u8]>, train: Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>) {
        if !train.is_empty() {
            self.task_family.insert(task, train);
        }
    }

    /// Speak a submitted rule onto withheld tests and compare cells exactly.
    /// The rule's cited forms resolve through this kernel's ruled cover.
    pub fn rule_matches_tests(&self, rule: &Element, tests: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> bool {
        if tests.is_empty() {
            return false;
        }
        let Some(sr) = sentence_rule_of(rule) else {
            return false;
        };
        tests.iter().all(|(x, y)| {
            crate::transducer::family::speak_rule(&sr, x, self).is_some_and(|s| s.grid() == *y)
        })
    }

    /// Pay a verified solve from the escrow of listings that task deflated.
    /// When a rule is supplied, the home speaks it on the withheld tests;
    /// a full match binds the solve to those listings, draws their escrow,
    /// and Offers the rule on the docket.
    pub fn pay_solve(
        &mut self,
        task: &[u8],
        voice: [u8; 32],
        mu: [u64; 4],
        rule: Option<&Element>,
        tests: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
    ) -> [u64; 4] {
        if let Some(rule) = rule {
            if !self.rule_matches_tests(rule, tests) {
                return [0; 4];
            }
            self.docket_connection(rule, voice);
            self.offered.insert(rule.address(), ());
        }
        let listings: Vec<Address> = self.task_listings.get(task).cloned().unwrap_or_default();
        let mut paid = [0u64; 4];
        let anchor = self.anchor();
        for addr in &listings {
            if let Some((_, held, a)) = self.treasury.release_award(addr) {
                let take = self.treasury.settle_solve(voice, held, a);
                for l in 0..4 {
                    paid[l] = paid[l].saturating_add(take[l]);
                }
            }
        }
        if paid.iter().all(|&x| x == 0) {
            return self.treasury.settle_solve(voice, mu, anchor);
        }
        paid
    }
}
