// 42, 23.7 — THE COURT'S STROKE: what a verdict does to the book, and the
// stroke that rules the docket.
//
//! A verdict is applied by replaying the word it names, never by trusting
//! the signature that carried it. What one stroke fills, pays and files is
//! `fills.rs`, on the same Budget.

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
        // 45.1: A RULING IS AN ADMITTED ACT. Every arm below moves the book
        // -- the word joins the ruled cover, or its award evaporates, or it
        // is excised and re-admitted as a cavity -- and none of it was a
        // step of this kernel's clock, so the chain held no position for the
        // act that changes the book most.
        //
        // Sealed HERE, at the one point past which the book has moved on
        // every path: the two refusals above return before the provisional
        // is removed and rule nothing, so they seal nothing; past the
        // removal every arm has already changed what this court holds, the
        // rewrite arm's `return false` included. The author is the court
        // whose verdict this is (`set_court_voice`, 47.2), and this court
        // itself when it is ruling on its own stroke.
        let ruler = self.court_voice.unwrap_or(self.identity);
        self.append_step(crate::chronometry::ActKind::Ruling, addr, ruler);
        if ruling == RULING_UPHELD {
            // Upheld on this court's own book: the word joins the ruled
            // cover AND becomes this court's section on the chart (13.9).
            self.rule_own(addr);
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
            let revoked = self.treasury.revoke_award(addr, slash);
            // The reading names the lanes on which a slashed voice's bond
            // did not cover the ask; the court writes it rather than
            // discarding it into the ruling's own return.
            if let Some(r) = &revoked {
                for (who, short) in &r.short {
                    for l in crate::economy::lanes_short(short) {
                        eprintln!(
                            "  court slash short voice {:02x}{:02x}… lane {l} by {}",
                            who[0], who[1], short[l]
                        );
                    }
                }
            }
            let ok = revoked.is_some();
            if slash {
                self.redocket_citers_of(addr);
                // A slashed word leaves the language and is admitted as a
                // cavity chart: the residual is a boundary, never garbage.
                if let Some(elem) = self.basis.get(addr).map(|w| w.element().clone()) {
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
    ///
    /// `lambda` is 47.5's Λ for a listing's chart, in admitted receipts,
    /// from the caller's measured transport to the hypha the Cover homes
    /// that chart to; it is asked with the chart, the listing's magnitude
    /// and this court's room, and the lapse does not fire where it reads
    /// `None`. See `lapse_docket`.
    pub fn court_stroke(
        &mut self,
        lambda: impl Fn(&crate::transducer::stalk::Invariant, &[u64; 4], &[u64; 4]) -> Option<u64>,
    ) {
        let mut budget = self.physics.stroke();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let from = self.open_cursor.clone();
        // index: budgeted keyed cursor over the open docket — an orbit
        // (45.7, S13.3): it continues past the last key from the first, so
        // a listing admitted below the cursor is ruled on the next circuit,
        // and the circuit it completes is the docket's orbit.
        let circuit = {
            let mut cur = self.open.cursor(&from, &mut budget);
            for (addr, _) in &mut cur {
                keys.push(addr);
            }
            if let Some(k) = cur.last_key() {
                self.open_cursor = k.to_vec();
            }
            cur.circuit()
        };
        if let Some((n, period)) = self.docket_orbit.stroke(u64::from(circuit)) {
            eprintln!("  court orbit docket {n} period {period}");
            // 45.9, S13.7, R7: the phase over the window, read off the
            // receipts and the cover and written; nothing stores or gates
            // on it. The window is an aperture of docket orbits, so the
            // reading is taken when the circuit count crosses a multiple
            // of the aperture — a docket that fits one budget completes a
            // circuit per stroke, and read per circuit its window was one
            // receipt or none (`comprehension 0/0` at period 1).
            let aperture = u64::from(crate::physics::aperture_cycle());
            // THE WINDOW, read once and used by both the phase and the
            // fold: one reading, two consumers (law 12).
            let window = n / aperture != n.saturating_sub(u64::from(circuit)) / aperture;
            if window {
                self.read_phase();
            }
            // 45.8, S13.9, S13.10: an orbit that completes while a lane is
            // short is the syntropic phase, and the fold is the ledger's
            // consolidation stroke in it — proposed here, carried to a
            // witness by the exchange, sealed on both signatures (47.3). A
            // prefix that is not final proposes nothing and writes no line.
            //
            // ONCE PER APERTURE OF ORBITS, NOT ONCE PER ORBIT. A docket
            // that fits one budget completes a circuit every stroke, so
            // this proposed a fold on every one of them where any lane was
            // short — which is most of them. Measured at epoch 12: 374
            // folds in six minutes, ~250 MiB a minute into the cold
            // segments, 620 MiB and 662 MiB in two and a half minutes, on
            // a root filesystem with 13 GiB left. An era is a
            // consolidation of the ledger, not a per-stroke act, and it is
            // paced by the same window the phase reading is — the aperture
            // of docket orbits this court has actually completed.
            if window
                && self.fold_pending.is_none()
                && !crate::economy::lanes_short(&self.physics.reserve_shortfall([true; 4])).is_empty()
            {
                let _ = self.propose_fold();
            }
        }
        for addr in keys {
            // R1 (45.7): the orbit rules what is not yet ruled. A listing on
            // the ruled cover or in the cavity is passed over — its standing
            // moves by a verdict or a lapse, never by the circuit. Replayed
            // on every circuit, one word was sectioned 20,800 times and
            // another slashed 2,227 times (2026-09-09).
            if self.ruled_contains(&addr) || self.cavity_book.contains(&addr) {
                continue;
            }
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
            // The stroke ruled a word THIS court replayed from its own
            // book: the line already calls it a section, and now it is one
            // (13.9, 13.10) -- a vantage a peer's section can be compared
            // against, which is what makes a glue a Mayer-Vietoris step.
            self.rule_own(&addr);
            eprintln!("  court section {:02x}{:02x}… ruled cover, escrow held", addr[0], addr[1]);
        }
        self.lapse_docket(&lambda);
        self.relocate_t_exhausted();
        self.stroke_fills(&mut budget);
    }

    /// The docket's orbit reading (45.7): circuits completed and the
    /// strokes the last one took, the court's local time beside K.
    pub fn docket_orbit(&self) -> crate::physics::Orbit {
        self.docket_orbit
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
        // A SPOKEN word, decoded by name (L16): it is what a voice sent,
        // not what this court holds.
        if let Some(f) = crate::transducer::sentence::from_element_form(spoken) {
            if !f.torsion.is_empty() {
                return f
                    .torsion
                    .fiber(crate::network::nerve::ROOT)
                    .map(|(k, filed)| {
                        let ell = filed.period;
                        let rotor = torsion_loop_rotor(&f, &k);
                        (Address::from(k), ell, rotor)
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

    /// Record a filled cell's gauge as a dislocation on each section's
    /// word. The gauge arrives as a ROTOR and a CARRIAGE (L11e); the
    /// dislocation is the whole transport, so the two are composed here
    /// rather than the placement being dropped.
    pub(crate) fn record_gamma<A: AsRef<[u8]>>(
        &mut self,
        addrs: &[A],
        gamma: &crate::transducer::alphabet::Rotor,
        carriage: &crate::transducer::element::Versor,
    ) {
        let Some(v) = rotor_versor(gamma).map(|r| carriage.geo(&r)) else {
            return;
        };
        for addr in addrs {
            let key = addr.as_ref();
            let Some(asg) = self.basis.get(key).map(|w| element::assignment(w.sub())) else {
                continue;
            };
            if let Some(w) = self.word_mut(key) {
                w.update(|e| e.record_dislocation(asg, v.clone(), element::FIBER_SPATIAL));
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
