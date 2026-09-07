// E2, 26 — SETTLEMENT: what each closed act pays, per lane.
//
//! One function per act the market closes. Each is a movement of value that
//! already exists or a mint against a measured deflation; none of them
//! prices anything, because the metabolism governs the rate and never the
//! price.

use super::*;

impl Treasury {
    /// The POW++ GATE on a germ install: it admits iff the germ reduces the demand
    /// (`0⃗ ≺_DM magnitude`). NB: installing pays NOTHING here — a germ that only
    /// closes its own birth demand has proven nothing about the corpus. Reward
    /// flows later, on USE (`settle_use`), proportional to real demand unlocked.
    pub fn settle_install(&mut self, g: &Element, _voice: [u8; 32]) -> bool {
        descends(&[0u64; 4], &magnitude(g))
    }

    /// PoUW — a word was USED: a real later demand decomposed through it. THE
    /// CIRCULATION LAW: the DEMANDER pays the word's DISCOVERER the per-lane
    /// structure exercised, from the demander's EARNED balance — a peer transfer,
    /// bounded by what the payer really holds. Knowledge is nonrival: a broke
    /// demander is still served; the discoverer earns what was real. A demander
    /// paying itself transfers nothing (no wash trades). Returns the paid μ⃗.
    /// Two `transfer` entries share the anchor: first debits the demander,
    /// second credits the discoverer. Neither is a mint.
    pub fn settle_use(&mut self, demander: [u8; 32], discoverer: [u8; 32], mag: [u64; 4], anchor: Anchor) -> [u64; 4] {
        if demander == discoverer {
            return [0; 4];
        }
        let held = self.balance_mu(&demander);
        let paid: [u64; 4] = std::array::from_fn(|l| mag[l].min(held[l]));
        if paid.iter().all(|&m| m == 0) {
            return paid;
        }
        self.entries.push(Entry { kind: "transfer", who: demander, mu: paid, k_step: anchor.0, receipt: anchor.1 });
        self.debit(demander, paid);
        self.entries.push(Entry { kind: "transfer", who: discoverer, mu: paid, k_step: anchor.0, receipt: anchor.1 });
        self.credit(discoverer, paid);
        paid
    }

    /// A SUBSIDY from the pool (system-initiated yields with no payer: the
    /// redundancy probe, bootstrap awards). Bounded by the pool, like all pay.
    pub fn settle_subsidy(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        self.mint_to(who, mu, anchor)
    }

    /// Yield of an optimisation, read on the carriers as a lane vector.
    /// Zero on every lane is not work — refused, not paid zero.
    pub fn settle_rewrite(
        &self,
        kind: crate::transducer::rewrite::RewriteKind,
        before: &[crate::transducer::element::Element],
        after: &crate::transducer::element::Element,
    ) -> Option<[u64; 4]> {
        let mu = crate::transducer::rewrite::yield_vector(kind, before, after);
        if mu.iter().all(|&x| x == 0) {
            None
        } else {
            Some(mu)
        }
    }

    /// Release an escrowed Rewrite: mint the claimed vector to the discoverer.
    /// Release an escrowed Rewrite to the parties the escrow names: EVERY
    /// voice that stood it, split per lane, as a consilience award is split.
    /// `escrow_award` accumulates voices — each new one is pushed onto the
    /// entry — so they are all parties to the claim, and paying
    /// `voices.first()` paid one of several who did the work, chosen by the
    /// order they happened to arrive in (rule 15). The `voice` argument it
    /// fell back on was an invented payee besides (rule 11).
    ///
    /// `None` when the address holds no escrow, or holds one with no voice:
    /// a mint with no party is a mint to nobody, and the escrow stands
    /// rather than being spent into the void.
    pub fn release_rewrite(&mut self, addr: &[u8]) -> Option<[u64; 4]> {
        let (voices, mu, anchor) = self.escrowed.get(addr)?;
        if voices.is_empty() {
            return None;
        }
        let (voices, mu, anchor) = (voices.clone(), *mu, *anchor);
        self.escrowed.remove(addr);
        let n = voices.len() as u64;
        let share: [u64; 4] = std::array::from_fn(|l| if mu[l] == 0 { 0 } else { (mu[l] / n).max(1) });
        for v in &voices {
            self.mint_to(*v, share, anchor);
        }
        Some(share)
    }

    pub fn settle_refine(&mut self, bidder: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        self.mint_to(bidder, mu, anchor)
    }

    pub fn settle_condense(&mut self, discoverer: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        self.mint_to(discoverer, mu, anchor)
    }

    pub fn settle_solve(&mut self, voice: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let asked: [u64; 4] = std::array::from_fn(|l| if mu[l] == 0 { 0 } else { mu[l].max(1) });
        self.mint_to(voice, asked, anchor)
    }

    /// THE ASSEMBLY PREMIUM (7d): a fragmented parent demand's children all
    /// completed and the overlap strips glued — distributed reassembly is
    /// substantially-more-useful work, paid ON TOP of the children's own
    /// settlements: μ⃗_composite = Σμ⃗(Wᵢ) + μ⃗_assembly(overlaps). Split per
    /// lane across the reassembling voices, float-law bounded like every award.
    pub fn settle_assembly<A: AsRef<[u8]>>(&mut self, voices: &[A], mu: [u64; 4], anchor: Anchor) {
        if voices.is_empty() {
            return;
        }
        let n = voices.len() as u64;
        let share: [u64; 4] = std::array::from_fn(|l| if mu[l] == 0 { 0 } else { (mu[l] / n).max(1) });
        for v in voices {
            let Ok(who) = <[u8; 32]>::try_from(v.as_ref()) else {
                continue;
            };
            self.mint_to(who, share, anchor);
        }
    }

    /// PoWc — split the consilience award (the germ's μ⃗ PRICED at the moment's
    /// surface) across the attesting voices, per lane (each nonzero lane pays at
    /// least 1 to each voice — consilience is never free).
    pub fn settle_consilience(&mut self, voices: &[[u8; 32]], mu: [u64; 4], anchor: Anchor) {
        if voices.len() < 2 {
            return;
        }
        let n = voices.len() as u64;
        let share: [u64; 4] = std::array::from_fn(|l| if mu[l] == 0 { 0 } else { (mu[l] / n).max(1) });
        for v in voices {
            self.mint_to(*v, share, anchor);
        }
    }

    pub fn settle_torse(&mut self, discoverer: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        self.mint_to(discoverer, mu, anchor)
    }

    /// A reading (decomposition) charges the READER a unit fee on each lane the
    /// sentence exercised — paid from the reader's earned balance INTO the pool.
    /// This is the RECHARGE stroke: consumption refills the subsidy float, so
    /// the pool breathes (drain on discovery, refill on use) instead of dying.
    pub fn settle_recall(&mut self, voice: [u8; 32], lanes: [bool; 4], anchor: Anchor) -> [u64; 4] {
        let held = self.balance_mu(&voice);
        let fee: [u64; 4] = std::array::from_fn(|l| (lanes[l] as u64).min(held[l]));
        if fee.iter().all(|&m| m == 0) {
            return fee;
        }
        self.entries.push(Entry { kind: "transfer", who: voice, mu: fee, k_step: anchor.0, receipt: anchor.1 });
        self.debit(voice, fee);
        fee
    }

    /// THE DOOR FEE (D12) — a strained mesh prices its own door: the speaker
    /// pays the congestion fee (per lane, derived from the reserve deficit —
    /// zero on a calm mesh) from earned balance INTO the pool. The door is an
    /// economic rate limit, not a gate: what cannot be paid is simply not
    /// paid, and the caller decides deferral from the shortfall. Returns the
    /// per-lane amount actually paid.
    pub fn settle_door(&mut self, voice: [u8; 32], fee: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let held = self.balance_mu(&voice);
        let paid: [u64; 4] = std::array::from_fn(|l| fee[l].min(held[l]));
        if paid.iter().all(|&m| m == 0) {
            return paid;
        }
        self.entries.push(Entry { kind: "transfer", who: voice, mu: paid, k_step: anchor.0, receipt: anchor.1 });
        self.debit(voice, paid);
        paid
    }

}
