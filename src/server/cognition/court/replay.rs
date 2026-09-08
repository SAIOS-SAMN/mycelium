// 42, 23.7 — THE COURT'S STROKE: what a verdict does to the book, and what
// one stroke of the court fills, pays and files.
//
//! A verdict is applied by replaying the word it names, never by trusting
//! the signature that carried it. A fill is paid by the volume of the cell
//! it closes; a torsion cycle is filed at the period it was witnessed at.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::{self, Element};

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
            if let Some(crate::economy::escrow::Escrowed { voices, mu, anchor }) =
                self.treasury.release_award(addr)
            {
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
                    self.cavity_book.insert(addr, elem.clone());
                    self.cavities.push(elem);
                }
                self.mark_persist();
            }
            ok
        }
    }

    /// The family a spoken form is bound to on this court (23.8): the
    /// trains of the task it was spoken for, or of the listing it answers.
    pub(crate) fn family_of(&self, addr: &[u8]) -> Option<&[crate::transducer::family::TrainPair]> {
        let task = self.speech_task.get(addr).or_else(|| self.listing_task.get(addr))?;
        self.task_family.get(task).map(Vec::as_slice)
    }

    /// THE COURT'S REPLAY of a word it holds (23.1, 23.8): through its own
    /// genome as the `Crystal` that resolves what the word cites, and
    /// against the family the word is bound to, so step 7 — closure on
    /// every pair, `REFUSE_NOT_CLOSED` — is checked on the court path and
    /// not only when the word was first spoken. A word bound to no family
    /// replays without that clause: it can still be ruled, and it mints
    /// only what it measures on a family, which is then nothing.
    pub(crate) fn court_replay(&self, spoken: &Element) -> Option<&'static str> {
        let family = self.family_of(&spoken.address());
        crate::verdict::blind_replay_on(spoken, &|a| self.resolve_form(a), family, self)
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
            if let Some(reason) = self.court_replay(&spoken) {
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

    pub(crate) fn stroke_fills(&mut self, budget: &mut crate::physics::Budget) {
        if budget.remaining() == 0 {
            return;
        }
        let mut pending: Vec<(crate::transducer::stalk::Invariant, crate::transducer::address::Address)> = Vec::new();
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
                            if let Some(row) = self.cocycle_at.remove(&key) {
                                self.dialect_chain.push((key.clone(), row.gauge, row.t_universal));
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

    pub(crate) fn volume_of_cell<A: AsRef<[u8]>>(&self, inv: &crate::transducer::stalk::Invariant, verts: &[A]) -> [u64; 4] {
        let held = self.listings_on(inv, verts);
        let mut chain = Vec::new();
        for (a, _) in held.fiber(crate::network::nerve::ROOT) {
            if let Some(f) = self.field.get(&a) {
                chain.extend(union_remainders(&f.entries));
            }
        }
        element::measure_lanes(&chain)
    }

    pub(crate) fn pay_cell_volume<A: AsRef<[u8]>>(&mut self, inv: &crate::transducer::stalk::Invariant, addrs: &[A]) {
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
    pub(crate) fn pay_torse<A: AsRef<[u8]>>(&mut self, addrs: &[A]) {
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

    pub(crate) fn file_torsion<A: AsRef<[u8]>>(&mut self, addrs: &[A], ell: u32) {
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
    pub(crate) fn mint_or_admit_translation(
        &mut self,
        gamma: &crate::transducer::alphabet::Rotor,
        a: crate::transducer::address::Address,
        b: crate::transducer::address::Address,
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
    pub(crate) fn court_torsion(
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
    pub(crate) fn spoken_torsion_cycles(
        spoken: &Element,
    ) -> Vec<(Address, u32, Option<crate::transducer::alphabet::Rotor>)> {
        use crate::transducer::sentence::torsion_loop_rotor;
        if let Some(f) = spoken.form() {
            if !f.torsion.is_empty() {
                return f
                    .torsion
                    .fiber(crate::network::nerve::ROOT)
                            .map(|(k, filed)| {
                                let ell = &filed.period;
                        let rotor = torsion_loop_rotor(f, &k);
                        (Address::from(k), *ell, rotor)
                    })
                    .collect();
            }
        }
        spoken
            .torsion()
            .iter()
            .filter_map(|c| {
                let r = spoken.letters()?.spelling.word.rotor();
                Some((r.class_key()?, c.period, Some(r)))
            })
            .collect()
    }

    pub(crate) fn record_gamma<A: AsRef<[u8]>>(&mut self, addrs: &[A], gamma: &crate::transducer::alphabet::Rotor) {
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

    pub(crate) fn mark_class(
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

    pub(crate) fn refine_chart(
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

}
