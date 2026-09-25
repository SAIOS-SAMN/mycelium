// 23.7, 42 — WHAT ONE STROKE OF THE COURT FILLS, PAYS AND FILES: the
// unfilled simplices and the pending refinements, the volume a fill is
// paid by, the torsion a congruent simplex files.
//
//! Split from `replay.rs` (MU-STANDARDS 1.1, one organ per file): the
//! court's stroke rules the docket there and fills here, on the same
//! stroke Budget.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element;

/// RM1 (c) — VOICES' RE-FILLS REPLAYED: applied (the record answered); and
/// refused — not the re-fill's cites, a word not located here, a spelling
/// word that does not spell the ray, a rewrite with no open remainder of the
/// class there. Of the applied: no Congruent simplex at the rewrite's `after`
/// in its chart's complex (the write skipped, the answer stands), and a
/// write the named door refused.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RefillTally {
    pub applied: u64,
    pub unshaped: u64,
    pub unlocated: u64,
    pub unspelled: u64,
    pub unopen: u64,
    pub unsited: u64,
    pub unfilled: u64,
}

impl Kernel {
    pub(crate) fn stroke_fills(&mut self, budget: &mut crate::physics::Budget) {
        if budget.remaining() == 0 {
            return;
        }
        let mut pending: Vec<(crate::transducer::stalk::Invariant, crate::transducer::address::Address)> = Vec::new();
        {
            // Resumed, never restarted at the first key (rule 10, S13.3):
            // a head that refills faster than one budget no longer starves
            // the tail, and the circuit reaches every simplex.
            let from = self.fill_cursor.clone();
            let mut cur = self.unfilled.cursor(from.as_deref(), budget);
            for (_, v) in &mut cur {
                pending.push(v.clone());
            }
            if let Some(k) = cur.last_key() {
                self.fill_cursor = Some(k.to_vec());
            }
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
                    .scoped()
                    .map(|(_, s)| s.addr.clone())
                    .collect();
                (cx.fill(&sim_key), addrs)
            };
            match filled {
                (
                    crate::transducer::cech::CoboundaryOutcome::Attached(
                        crate::transducer::cech::Fill::Filled(gamma, carriage),
                    ),
                    addrs,
                ) => {
                    eprintln!(
                        "  court simplex filled {:02x}{:02x}… γ recorded",
                        sim_key[0], sim_key[1]
                    );
                    self.record_gamma(&addrs, &gamma, &carriage);
                    let before = self.chain.head_stamp();
                    self.pay_simplex_volume(&inv, &addrs);
                    // 78.27 (5): the Settled act this fill sealed, where it
                    // sealed one — the act a transition it retires stood at
                    // when the transition carries no receipt of its own.
                    let settled = self.chain.head_stamp().filter(|h| Some(*h) != before);
                    if !gamma.is_identity() {
                        if let Some((a, b)) = edge_pair(&addrs) {
                            self.mint_or_admit_translation(&gamma, a.clone(), b.clone(), [0; 32]);
                            let key = Self::cocycle_key(&inv, &a, &b);
                            if let Some(row) = self.cocycle_at.remove(&key) {
                                let stood = self.cocycle_receipt.get(&key).copied().or(settled);
                                self.dialect_chain.push((key.clone(), row.gauge, stood));
                            }
                            if let Some(m) = self.cocycle_by_chart.get_mut(&inv.key()) {
                                m.remove(&key);
                            }
                        }
                    }
                }
                (
                    crate::transducer::cech::CoboundaryOutcome::Attached(
                        crate::transducer::cech::Fill::Congruent(kappa, g, ell),
                    ),
                    addrs,
                ) => {
                    eprintln!(
                        "  court simplex congruent {:02x}{:02x}… torsion filed",
                        sim_key[0], sim_key[1]
                    );
                    self.pay_simplex_volume(&inv, &addrs);
                    // RM1 — the Torse rewrite minted with its remainder; the
                    // torsion paid cites it.
                    if let Some(rw) = self.mint_torse_at(&inv, sim_key.as_bytes(), (kappa, g, ell), &addrs) {
                        self.pay_torse(&addrs, rw.as_bytes());
                    }
                }
                (
                    crate::transducer::cech::CoboundaryOutcome::Attached(
                        crate::transducer::cech::Fill::Class(_, class),
                    ),
                    _,
                ) => {
                    let class = *class;
                    eprintln!(
                        "  court simplex class {:02x}{:02x}…",
                        sim_key[0], sim_key[1]
                    );
                    let degree = self
                        .complexes
                        .get(&inv.key())
                        .and_then(|c| c.class(&sim_key).map(|(_, _, d)| d))
                        // A class the complex does not hold stands at the
                        // edge degree, which is where a fill first names one.
                        .map_or(2, |d| d);
                    self.mark_class(&inv, class.clone(), degree);
                    if degree >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class, degree));
                    }
                }
                // 37.7 AT THE COURT: a simplex whose transition is a
                // rotation the seventeen do not spell. It fills nothing and
                // files no class, and it is the court's own adjunction
                // candidate — named on its own line so the day one appears
                // is a measurement and not an anecdote.
                //
                // IT IS NOT MINTED HERE, and that is not an omission. This is
                // a sweep over `self.unfilled` with no speaking voice in it,
                // and a minted letter is docketed to a DISCOVERER — 47.3, a
                // court is never one. The simplices this sweep reads were
                // put in `unfilled` by `glue_fetched`, which has the peer
                // whose speech produced them and adjoins there. Minting here
                // would have to name a party that does not exist.
                (
                    crate::transducer::cech::CoboundaryOutcome::Attached(
                        crate::transducer::cech::Fill::Unspellable(_),
                    ),
                    _,
                ) => {
                    eprintln!(
                        "  court simplex unspellable {:02x}{:02x}… 37.7",
                        sim_key[0], sim_key[1]
                    );
                }
                // Evaluated and not measured, by its reason (78.26(5)).
                (
                    crate::transducer::cech::CoboundaryOutcome::Attached(
                        crate::transducer::cech::Fill::Unread(why),
                    ),
                    _,
                ) => {
                    eprintln!("  court simplex unread {:02x}{:02x}… {why:?}", sim_key[0], sim_key[1]);
                }
                // Evaluated and still open, already settled, or not there
                // to evaluate — three different facts, none of which this
                // sweep acts on.
                (
                    crate::transducer::cech::CoboundaryOutcome::Attached(
                        crate::transducer::cech::Fill::Open,
                    ),
                    _,
                )
                | (crate::transducer::cech::CoboundaryOutcome::AlreadyFilled, _)
                | (crate::transducer::cech::CoboundaryOutcome::Absent, _) => {}
            }
        }
        let mut refine: Vec<(
            crate::transducer::stalk::Invariant,
            crate::transducer::alphabet::Rotor,
            usize,
        )> = Vec::new();
        {
            let from = self.refine_cursor.clone();
            let mut cur = self.pending_refine.cursor(from.as_deref(), budget);
            for (_, v) in &mut cur {
                refine.push(v.clone());
            }
            if let Some(k) = cur.last_key() {
                self.refine_cursor = Some(k.to_vec());
            }
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

    /// R6 (47.3): the volume of a simplex is the measure of the open
    /// listings on its chart. Its vertices are vantages — hyphas — and no
    /// speaker's index gates it, so the simplex's vertex list is not an
    /// argument here.
    pub(crate) fn volume_of_simplex(&self, inv: &crate::transducer::stalk::Invariant) -> [u64; 4] {
        let held = self.listings_on(inv);
        let mut chain = Vec::new();
        for (a, _) in held.scoped() {
            if let Some(f) = self.field.get(&a) {
                chain.extend(union_remainders(&f.entries));
            }
        }
        element::measure_lanes(&chain)
    }

    pub(crate) fn pay_simplex_volume<A: AsRef<[u8]>>(&mut self, inv: &crate::transducer::stalk::Invariant, addrs: &[A]) {
        let held = self.listings_on(inv);
        let vol = self.volume_of_simplex(inv);
        // Every vertex of the simplex is a party to what the simplex
        // closed. The volume is one quantity for all of them, so it splits
        // per lane as a consilience award does; paying the first address
        // would pay whichever member the simplex happens to list first
        // (rule 15).
        let parties: Vec<[u8; 32]> = addrs
            .iter()
            .filter_map(|a| <[u8; 32]>::try_from(a.as_ref()).ok())
            .collect();
        if parties.is_empty() {
            return;
        }
        let n = parties.len() as u64;
        let share: [u64; 4] = std::array::from_fn(|l| vol[l] / n);
        if share.iter().all(|&m| m == 0) {
            return;
        }
        let vol = share;
        let me = self.identity;
        for (a, _) in held.scoped() {
            // EC1: each drawn escrow is a Settled act of this court citing
            // the escrow's listing, sealed first; its POUW rows stand at it.
            if self.treasury.escrow_of(&a).is_none() {
                continue;
            }
            self.append_step(crate::chronometry::ActKind::Settled, &a, me);
            let anchor = self.anchor();
            for v in &parties {
                let _ = self.treasury.draw_escrow(&a, *v, vol, anchor);
            }
        }
    }

    /// Each member of the simplex is paid the torsion IT measured. Reading
    /// the magnitude off the first address would make the value of a
    /// torsion depend on which member the simplex happens to list first,
    /// and pay it to that member alone; a member with no measured tension
    /// is unmeasured and is paid nothing, never paid another member's
    /// reading (rule 11, 15).
    pub(crate) fn pay_torse<A: AsRef<[u8]>>(&mut self, addrs: &[A], rewrite: &[u8]) {
        // EC1: the torsion paid is a Settled act of this court citing the
        // Torse rewrite the fill minted (RM1: never a word chosen from the
        // simplex), sealed first; its POUW rows stand at it.
        let parties: Vec<([u8; 32], [u64; 4])> = addrs
            .iter()
            .filter_map(|a| {
                let who = <[u8; 32]>::try_from(a.as_ref()).ok()?;
                let mag = self.tension.get(a.as_ref()).copied()?;
                Some((who, mag))
            })
            .collect();
        // A simplex no member of which measured a tension pays nothing and
        // seals nothing.
        if parties.is_empty() {
            return;
        }
        let me = self.identity;
        self.append_step(crate::chronometry::ActKind::Settled, rewrite, me);
        let anchor = self.anchor();
        for (who, mag) in parties {
            let _ = self.treasury.settle_torse(who, mag, anchor);
        }
    }

    /// RM1 (c) — A VOICE'S RE-FILL, REPLAYED: its Settled act cites the
    /// rewrite word, `Cite::Class {κ, g, cover: the chain}` and the word
    /// that spells the twist. The court locates both words, re-reads the
    /// spelling word's rotor against `g` and the rewrite's open remainder of
    /// κ at that chain, and only then writes its Congruent simplex — at the
    /// rewrite's `after` in the complex of the chain's chart — `Filled`
    /// through the named door; the rewrite's remainder is answered at the
    /// act's stamp, and the pressure it answered is minted to the voice under
    /// POUW. Nothing else is paid. Each refusal is counted by why.
    pub(crate) fn hear_refill(&mut self, r: &crate::chronometry::KStepReceipt) {
        use crate::chronometry::Cite;
        let [Cite::Word { chart: c1, class: k1, at: a1 }, Cite::Class { class, rotor, cover: Some(at) }, Cite::Word { chart: c2, class: k2, at: a2 }] =
            r.cites.as_slice()
        else {
            self.refills.unshaped += 1;
            return;
        };
        let locate = |k: &Self, chart, cls, at| {
            k.word_admitted_at(at).cloned().or_else(|| match k.fiber_read(chart, cls).as_slice() {
                [one] => Some(one.clone()),
                _ => None,
            })
        };
        let (Some(rw_addr), Some(sp_addr)) = (locate(self, c1, k1, a1), locate(self, c2, k2, a2)) else {
            self.refills.unlocated += 1;
            return;
        };
        let (Some(rewrite), Some(spelling)) =
            (self.named_word(rw_addr.as_bytes()).cloned(), self.named_word(sp_addr.as_bytes()).cloned())
        else {
            self.refills.unlocated += 1;
            return;
        };
        // The spelling word spells the twist: its word vertex's derivation
        // re-read, projectively the cited ray.
        let spells = crate::transducer::sentence::from_element_form(&spelling).is_some_and(|f| {
            f.vertices.iter().any(|v| {
                matches!(v, crate::transducer::sentence::Vertex::Word { derivation, .. } if derivation.rotor().projective_eq(rotor))
            })
        });
        if !spells {
            self.refills.unspelled += 1;
            return;
        }
        let open = rewrite
            .remainders()
            .any(|o| *o.class == **class && o.at == *at && o.answered.is_none() && o.ray.projective_eq(rotor));
        let Some(mu) = rewrite.open_pressure(class, at).filter(|_| open) else {
            self.refills.unopen += 1;
            return;
        };
        // The act replays whole: the rewrite's record is answered on
        // whichever row holds it — the basis, the provisional row it was
        // offered on, or the listing book — whether or not a simplex stands
        // here to write, so the court's record and the mind's never diverge.
        let key = rw_addr.as_bytes();
        let answered = match self.word_mut(key) {
            Some(wm) => wm.update(|e| e.answer_remainder(class, at, r.stamp)),
            None => match self.provisional.get_mut(key) {
                Some(e) => e.answer_remainder(class, at, r.stamp),
                None => self.pending.get_mut(key).and_then(|e| e.answer_remainder(class, at, r.stamp)),
            },
        };
        if answered.is_none() {
            self.refills.unopen += 1;
            return;
        }
        // Only the complex's write is skipped where no Congruent simplex
        // stands at the rewrite's `after` (a mind's own rewrite, its `after`
        // the merged thought), counted apart from the answer; a write the
        // door refused is counted too (LD0). The court's own rewrite sits at
        // one chart, its complex's; a chain of several is a mind's.
        let site = match (crate::transducer::rewrite::Rewrite::from_element(&rewrite), at.as_slice()) {
            (Some(rw), [chart]) => Some((chart.clone(), rw.after.addr().as_bytes().to_vec())),
            _ => None,
        };
        let congruent = site.as_ref().is_some_and(|(chart, after)| {
            self.complexes
                .get(chart)
                .and_then(|cx| cx.simplex(after))
                .is_some_and(|s| matches!(s.fill, crate::transducer::cech::Fill::Congruent(..)))
        });
        match (congruent, site) {
            (true, Some((chart, after))) => {
                if self.complexes.get_mut(&chart).and_then(|cx| cx.refill(&after, (**rotor).clone())).is_none() {
                    self.refills.unfilled += 1;
                }
            }
            _ => self.refills.unsited += 1,
        }
        let anchor = self.anchor();
        self.treasury.mint_claim(&[r.stamp.vk], mu, anchor);
        self.refills.applied += 1;
    }

    // `file_torsion` STOOD HERE AND IS RETIRED (RM1, 78.37(4)). It recorded
    // `(σ, ℓ)` on the simplex's least address by key — a representative
    // chosen from the simplex (checklist (r)) — and dropped κ and g. The
    // remainder is now recorded whole on the Torse rewrite the fill mints.

    /// RM1, 78.37(4) — A SIMPLEX CLOSED UP TO ITS CLASS. The court mints the
    /// Torse rewrite carrying the simplex's words to their class: `before`
    /// the words, `after` the simplex's own key — its sections' coordinates
    /// (KB1), the coordinate of the object glued, never a word chosen from it
    /// — its payload σ and ℓ; and records the remainder (κ, g, the complex's
    /// chart) on the rewrite's own record, priced off that record. `None`,
    /// each counted by its own name, where nothing is priced (`unpriced`),
    /// the mint refuses — a name past its wall (`unminted`) — or the book
    /// does not take it (`unoffered`).
    pub(crate) fn mint_torse_at(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        simplex: &[u8],
        remainder: (Box<crate::transducer::sentence::ClassVector>, Box<crate::transducer::alphabet::Rotor>, u32),
        addrs: &[crate::transducer::address::Address],
    ) -> Option<crate::transducer::address::Address> {
        let (kappa, g, ell) = remainder;
        let cycle = element::TorsionCycle {
            sigma: Vec::new(),
            period: u64::from(ell),
            fibre: element::FIBER_SPATIAL,
            remainder: Some(element::Remainder { class: kappa, ray: g, at: vec![inv.key().into_vec()], answered: None }),
        };
        let Some(mu) = self.treasury.price_torse(std::slice::from_ref(&cycle)) else {
            self.remainders_unpriced += 1;
            return None;
        };
        let payload = crate::transducer::rewrite::RewritePayload::Torse { sigma: Vec::new(), period: u64::from(ell) };
        let after = crate::transducer::address::Address::from(simplex.to_vec());
        let Some(mut rw) = crate::transducer::rewrite::mint(crate::transducer::rewrite::RewriteKind::Torse, addrs, after, payload, mu)
        else {
            self.remainders_unminted += 1;
            return None;
        };
        match rw.record_cycle(cycle) {
            element::Filed::Refused => self.remainders_refused += 1,
            element::Filed::PastCarrier => self.remainders_past_carrier += 1,
            _ => {}
        }
        let addr = rw.address();
        let me = self.identity;
        if !self.offer_rewrite(rw, me) {
            self.remainders_unoffered += 1;
            return None;
        }
        Some(addr)
    }
}
