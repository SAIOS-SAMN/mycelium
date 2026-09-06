// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ECONOMY — THE DIMENSIONAL LEDGER (alpha Step 4). Every balance and every
//! receipt is a polytonic μ⃗ 4-vector (𝒮, 𝒢, 𝒯, 𝒽) — scalar amounts are
//! BANNED (a scalar balance is a flattened ledger). Every entry is ANCHORED to
//! the chronometric chain (K-step + head hash — the unforgeable temporal
//! binding; Step 5's court audits these). The treasury is a QUOTING ENGINE:
//! yield follows the surface (pressure + reserve deficit + entropic excess) and
//! the cohesion pair (C, I) differentiates the market — closure work prices by
//! C, coverage work by I, never summed. PoUW = paid decomposition of real open
//! demand (settle_use); PoWc = the ≺_DM descent gate + consilience receipts
//! sealed on the chain. Torsion extraction (torse) is the strongest coin —
//! structure condensed from disorder. Redundancy is structurally suppressed by
//! puncture/condense (same-geometry siblings cannot coexist as cards), which is
//! the anti-Goodhart inverse-redundancy law realized in the carrier itself.

use crate::transducer::element::Element;

/// A germ's polytonic magnitude — per-lane support size (‖Δ‖'s |supp| component),
/// never summed, computed EXACTLY from the germ's description. 𝒮 = motion support
/// (shift/rotation/scale), 𝒢 = relabeled-hue count, 𝒯 = temporal extent,
/// 𝒽 = holonomic volume. A settled demand's residual magnitude is the zero vector; a real
/// germ's is nonzero — hence the descent.
pub fn magnitude(g: &Element) -> [u64; 4] {
    g.magnitude()
}

/// The per-lane Dershowitz–Manna descent: `after ≺_DM before` iff after ≤ before
/// on every lane and strictly less on some lane. No lane is traded for another.
pub fn descends(after: &[u64; 4], before: &[u64; 4]) -> bool {
    after.iter().zip(before).all(|(a, b)| a <= b) && after.iter().zip(before).any(|(a, b)| a < b)
}

/// The unique index of the vector that equals the lane-wise minimum, if
/// there is exactly one. Several that match, or none (incomparable lanes),
/// are not dominant.
pub fn dominant(vals: &[[u64; 4]]) -> Option<usize> {
    let min: [u64; 4] = [
        vals.iter().map(|v| v[0]).min()?,
        vals.iter().map(|v| v[1]).min()?,
        vals.iter().map(|v| v[2]).min()?,
        vals.iter().map(|v| v[3]).min()?,
    ];
    let mut found = None;
    for (i, v) in vals.iter().enumerate() {
        if *v == min {
            if found.is_some() {
                return None;
            }
            found = Some(i);
        }
    }
    found
}

/// A refinement is measured as deflation on the split strata. No new price.
pub fn split_deflation(before: &[u64; 4], after: &[u64; 4]) -> Option<[u64; 4]> {
    if !descends(after, before) {
        return None;
    }
    Some(std::array::from_fn(|l| before[l] - after[l]))
}

/// THE YIELD SURFACE — the treasury's dynamic per-lane quote, read from GENUINE
/// metabolic tension: open pressure (unmet demand backlog) + reserve deficit
/// (the depletion premium) + entropic excess (torsion outrunning syntropy).
/// Strained lanes quote high; settled lanes quote low — never a static faucet,
/// and never a summed scalar (polytonic, #34).
pub fn yield_surface(p: &crate::physics::Physics) -> [u64; 4] {
    let flux = p.order_flux();
    let deficit = p.reserve_deficit();
    std::array::from_fn(|l| {
        let entropic = (-flux[l]).max(0) as u64;
        p.pressure[l] + deficit[l] + entropic
    })
}

/// PRICE IS THE MAGNITUDE ON THE LANE: a verified word is worth what it
/// deflated. The surface paces pressure, it does not scale the mint.
pub fn price(mag: &[u64; 4], _surface: &[u64; 4]) -> [u64; 4] {
    *mag
}

/// The chronometric anchor of a ledger entry: (K-step, chain head hash).
pub type Anchor = (u64, [u8; 32]);

/// Economic tiers for a decentralized mesh running cryptographic semantic
/// operations: yields scale by the participant's tier. A hypha/client's standing
/// weights its reward, so the topology supports staked, tiered roles — never a
/// flat one-class network.
/// Persistence discriminant for older snapshots. Not a value, not a scale,
/// not an enroll argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Tier {
    Founder,
    Validator,
    Participant,
    Observer,
}
/// A settlement receipt (below-boundary storage — a Vec, not a map): a
/// polytonic μ⃗ amount, chronometrically anchored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: &'static str,
    pub who: [u8; 32],
    /// The per-lane vector amount — never a scalar.
    pub mu: [u64; 4],
    /// K-step at booking.
    pub k_step: u64,
    /// The chain head hash at booking (the unforgeable temporal binding).
    pub receipt: [u8; 32],
}

/// THE FLOW ORIENTATION of a ledger entry relative to its identity — a signed
/// coefficient in the flow algebra (the chain-complex orientation), never a
/// boolean gate: +1 credits the identity, −1 debits it, 0 orients nothing
/// (markers whose μ⃗ moved the stake bond, not a balance). An exhaustive total
/// function — an unknown kind orients nothing rather than silently crediting.
/// Transfer credits are the second of a paired pair (same μ⃗, K, receipt);
/// `recompute_balances` applies those, not this sign.
pub fn orientation(kind: &str) -> i8 {
    match kind {
        "mint" => 1,
        "transfer" | "slash" => -1,
        _ => 0,
    }
}

/// Default operator bond when none is chosen — one unit on every lane.
pub const UNIT_BOND: [u64; 4] = [1, 1, 1, 1];

#[derive(Debug, Default, Clone)]
pub struct Treasury {
    entries: Vec<Entry>,
    /// Sum of live identity bonds. Escrowed awards are `escrowed`, never this.
    bond_total: [u64; 4],
    /// Enrolled identities and their staked μ⃗, keyed by identity.
    stakes: crate::network::nerve::Keyed<[u64; 4]>,
    /// AWARDS IN ESCROW (Step 5): consilience settlements held unfinalized —
    /// keyed by germ address. Only the court's verdict releases or revokes.
    escrowed: crate::network::nerve::Keyed<(Vec<[u8; 32]>, [u64; 4], Anchor)>,
    /// Per-voice balances, updated on every entry. Never a scan of the ledger.
    balances: crate::network::nerve::Keyed<[u64; 4]>,
}

impl Treasury {
    pub fn new() -> Self {
        Treasury::default()
    }

    fn credit(&mut self, who: [u8; 32], mu: [u64; 4]) {
        let mut b = self.balances.get(&who).copied().unwrap_or([0; 4]);
        for l in 0..4 {
            b[l] = b[l].saturating_add(mu[l]);
        }
        self.balances.insert(who, b);
    }

    fn debit(&mut self, who: [u8; 32], mu: [u64; 4]) {
        let mut b = self.balances.get(&who).copied().unwrap_or([0; 4]);
        for l in 0..4 {
            b[l] = b[l].saturating_sub(mu[l]);
        }
        self.balances.insert(who, b);
    }

    /// ENROLL an identity with an operator-chosen per-lane bond. Tops up
    /// when the recorded stake is below `bond`. Pool is the sum of bonds;
    /// leftover escrow history does not enter it. Returns the vector added
    /// this call.
    pub fn enroll(&mut self, who: [u8; 32], bond: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let want = bond;
        let held = self.stakes.get(&who).copied().unwrap_or([0; 4]);
        let add: [u64; 4] = std::array::from_fn(|l| want[l].saturating_sub(held[l]));
        if add.iter().any(|&x| x > 0) {
            self.stakes.insert(who, want);
            self.entries.push(Entry { kind: "stake", who, mu: add, k_step: anchor.0, receipt: anchor.1 });
        } else if self.stakes.get(&who).is_none() {
            self.stakes.insert(who, want);
        }
        self.refresh_bond();
        add
    }

    fn refresh_bond(&mut self) {
        let mut e = [0u64; 4];
        crate::snapshot::persist(&self.stakes, |_, mu| {
            for l in 0..4 {
                e[l] = e[l].saturating_add(mu[l]);
            }
        });
        self.bond_total = e;
    }

    /// Bond recorded for an identity, if enrolled. None is unbonded.
    pub fn bond_of(&self, who: &[u8; 32]) -> Option<[u64; 4]> {
        self.stakes.get(who).copied()
    }

    /// Bond recorded for an identity, if enrolled.
    pub fn stake_of(&self, who: &[u8; 32]) -> Option<[u64; 4]> {
        self.bond_of(who)
    }

    /// Sum of live stakes. Legacy entries are excluded.
    pub fn bond_total(&self) -> [u64; 4] {
        self.bond_total
    }

    /// Stakes whose identity is not in the live genesis become `legacy`
    /// entries and leave `stakes` / `bond_total`.
    pub fn bind_live(&mut self, live: &[[u8; 32]]) {
        let mut drop: Vec<([u8; 32], [u64; 4])> = Vec::new();
        crate::snapshot::persist(&self.stakes, |who, mu| {
            let Ok(w) = <[u8; 32]>::try_from(who) else { return };
            if !live.iter().any(|id| *id == w) {
                drop.push((w, *mu));
            }
        });
        for (who, mu) in drop {
            self.stakes.remove(&who);
            self.entries.push(Entry {
                kind: "legacy",
                who,
                mu,
                k_step: 0,
                receipt: [0; 32],
            });
        }
        self.refresh_bond();
    }

    /// Sum of mint entries, per lane — issued supply.
    pub fn supply(&self) -> [u64; 4] {
        let mut s = [0u64; 4];
        for e in &self.entries {
            if e.kind == "mint" {
                for l in 0..4 {
                    s[l] = s[l].saturating_add(e.mu[l]);
                }
            }
        }
        s
    }

    fn mint_to(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        if mu.iter().all(|&x| x == 0) {
            return [0; 4];
        }
        self.entries.push(Entry { kind: "mint", who, mu, k_step: anchor.0, receipt: anchor.1 });
        self.credit(who, mu);
        eprintln!(
            "  mint {:02x}{:02x}… μ⃗ {mu:?} k {}",
            who[0], who[1], anchor.0
        );
        mu
    }

    /// HOLD a consilience award in escrow pending the court's finality verdict
    /// (Step 5: two-voice consilience is provisional, not final). Nothing books
    /// to the ledger yet — unfinalized speech earns nothing.
    pub fn escrow_award(&mut self, addr: impl AsRef<[u8]>, voices: Vec<[u8; 32]>, mu: [u64; 4], anchor: Anchor) {
        let addr = addr.as_ref();
        match self.escrowed.get_mut(addr) {
            Some((held, _, _)) => {
                for v in voices {
                    match held.iter().position(|h| *h == v) {
                        Some(_) => {}
                        None => held.push(v),
                    }
                }
            }
            None => {
                self.escrowed.insert(addr, (voices, mu, anchor));
            }
        }
    }

    pub fn escrow_of(&self, addr: &[u8]) -> Option<&(Vec<[u8; 32]>, [u64; 4], Anchor)> {
        self.escrowed.get(addr)
    }

    pub fn escrowed_awards(&self) -> Vec<(Vec<u8>, Vec<[u8; 32]>, [u64; 4], Anchor)> {
        self.escrowed
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, (v, mu, an))| (a, v.clone(), *mu, *an))
            .collect()
    }

    /// A filled triangle releases the escrow of the listings its dialects settled.
    pub fn release_filled<A: AsRef<[u8]>>(&mut self, listings: &[A]) -> [u64; 4] {
        let mut released = [0u64; 4];
        for addr in listings {
            if let Some((voices, mu, anchor)) = self.release_award(addr.as_ref()) {
                self.mint_claim(&voices, mu, anchor);
                for l in 0..4 {
                    released[l] = released[l].saturating_add(mu[l]);
                }
            }
        }
        released
    }

    /// Clear the listing claim. Payment is a mint of the same vector, done
    /// by the caller after this returns (release then mint).
    pub fn release_award(&mut self, addr: &[u8]) -> Option<(Vec<[u8; 32]>, [u64; 4], Anchor)> {
        self.escrowed.remove(addr)
    }

    /// Mint a released claim: consilience split when two voices stand, else
    /// the first voice takes the vector.
    pub fn mint_claim(&mut self, voices: &[[u8; 32]], mu: [u64; 4], anchor: Anchor) {
        if voices.len() >= 2 {
            self.settle_consilience(voices, mu, anchor);
        } else if let Some(&v) = voices.first() {
            self.mint_to(v, mu, anchor);
        }
    }

    /// Drop the listing claim. A slash debits the stake bond and mints nothing.
    pub fn revoke_award(&mut self, addr: &[u8], slash: bool) -> bool {
        let Some((voices, mu, anchor)) = self.escrowed.remove(addr) else {
            return false;
        };
        if slash {
            let who = voices.first().copied().unwrap_or([0; 32]);
            if let Some(&held) = self.stakes.get(&who) {
                let next: [u64; 4] = std::array::from_fn(|l| held[l].saturating_sub(mu[l]));
                self.stakes.insert(who, next);
            }
            self.entries.push(Entry { kind: "slash", who, mu, k_step: anchor.0, receipt: anchor.1 });
            self.refresh_bond();
        }
        true
    }

    pub fn stakes(&self) -> Vec<([u8; 32], [u64; 4])> {
        self.stakes
            .fiber(crate::network::nerve::ROOT)
            .filter_map(|(w, mu)| <[u8; 32]>::try_from(w.as_slice()).ok().map(|who| (who, *mu)))
            .collect()
    }

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
    pub fn release_rewrite(&mut self, addr: &[u8], voice: [u8; 32]) -> [u64; 4] {
        let Some((voices, mu, anchor)) = self.escrowed.remove(addr) else {
            return [0; 4];
        };
        let who = voices.first().copied().unwrap_or(voice);
        self.mint_to(who, mu, anchor)
    }

    pub fn settle_refine(&mut self, bidder: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        self.mint_to(bidder, mu, anchor)
    }

    pub fn settle_condense(&mut self, discoverer: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        self.mint_to(discoverer, mu, anchor)
    }

    /// A VERIFIED SOLVE (PoUW): the registry confirmed a prediction against
    /// its held answers and named the task's tension. Occupied lanes pay at
    /// least 1. This is syntropic mining — the bounty is minted as the
    /// task's locked tension, never drawn from the stake bond.
    /// Draw a verified-solve payment from an escrowed listing award, so
    /// reduction and solve share one ledger. Remaining escrow stays for
    /// consilience release.
    pub fn draw_escrow(&mut self, addr: &[u8], voice: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let Some((_, held, _)) = self.escrowed.get(addr) else {
            return [0; 4];
        };
        let asked: [u64; 4] = std::array::from_fn(|l| if mu[l] == 0 { 0 } else { mu[l].max(1) });
        let take: [u64; 4] = std::array::from_fn(|l| asked[l].min(held[l]));
        if take.iter().all(|&m| m == 0) {
            return take;
        }
        if let Some((_, held, _)) = self.escrowed.get_mut(addr) {
            for l in 0..4 {
                held[l] -= take[l];
            }
        }
        self.mint_to(voice, take, anchor)
    }

    #[cfg(test)]
    pub fn escrowed_remaining(&self, addr: &[u8]) -> [u64; 4] {
        self.escrowed.get(addr).map(|(_, mu, _)| *mu).unwrap_or([0; 4])
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
            let who = <[u8; 32]>::try_from(v.as_ref()).unwrap_or([0; 32]);
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

    /// The POLYTONIC BALANCE — per-lane orientation-weighted sums; there is no
    /// scalar balance. Mint grows supply; the stake bond is independent.
    pub fn balance_mu(&self, who: &[u8; 32]) -> [u64; 4] {
        self.balances.get(who).copied().unwrap_or([0; 4])
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Whether a mint entry already stands for this identity.
    pub fn has_mint(&self, who: &[u8; 32]) -> bool {
        self.entries.iter().any(|e| e.kind == "mint" && e.who == *who)
    }

    /// Serialize the dimensional ledger: `[n][tag u8][who 32][μ⃗ 4×u64][k u64]
    /// [receipt 32]…` — vector amounts and chronometric anchors survive restart.
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for p in &self.bond_total {
            out.extend_from_slice(&p.to_le_bytes());
        }
        out.extend_from_slice(&(self.stakes.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.stakes, |who, mu| {
            out.extend_from_slice(who);
            out.push(0); // former tier discriminant; unused
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
        });
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {
            let tag: u8 = match e.kind {
                "mint" => 14,
                "stake" => 4,
                "slash" => 8,
                "transfer" => 15,
                "utility" | "consilience" | "torse" | "assembly" | "solve" | "refine" | "condense" | "rewrite" => 14,
                "spend" | "fee" => 15,
                _ => 255,
            };
            out.push(tag);
            out.extend_from_slice(&e.who);
            for m in &e.mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
            out.extend_from_slice(&e.k_step.to_le_bytes());
            out.extend_from_slice(&e.receipt);
        }
        // ESCROWED AWARDS section (Step 5) — appended after entries so older
        // snapshots (which simply end here) restore with an empty docket.
        out.extend_from_slice(&(self.escrowed.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.escrowed, |addr, (voices, mu, anchor)| {
            let _ = crate::transducer::a21::put_addr(&mut out, addr);
            out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
            for v in voices {
                out.extend_from_slice(v);
            }
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
            out.extend_from_slice(&anchor.0.to_le_bytes());
            out.extend_from_slice(&anchor.1);
        });
        out
    }

    /// Reload a ledger written by [`Treasury::snapshot`]. `None` on corruption.
    pub fn restore(b: &[u8], pos: &mut usize) -> Option<Treasury> {
        let mut bond_total = [0u64; 4];
        for p in &mut bond_total {
            *p = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let ns = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        if ns > b.len().saturating_sub(*pos) / 65 {
            return None; // untrusted count: 65 bytes per stake
        }
        let mut stakes = crate::network::nerve::Keyed::new();
        for _ in 0..ns {
            let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            *pos += 1; // former tier discriminant
            let mut mu = [0u64; 4];
            for m in &mut mu {
                *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            stakes.insert(who, mu);
        }
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        if n > b.len().saturating_sub(*pos) / 105 {
            return None; // untrusted count: 105 bytes per entry, refuse before allocating
        }
        let mut entries = Vec::with_capacity(n);
        for _ in 0..n {
            let tag = *b.get(*pos)?;
            *pos += 1;
            let kind = match tag {
                4 => "stake",
                8 => "slash",
                14 => "mint",
                15 => "transfer",
                0 | 1 | 3 | 9 | 10 | 11 | 12 | 13 => "mint",
                5 | 6 => "transfer",
                _ => "unknown",
            };
            let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            let mut mu = [0u64; 4];
            for m in &mut mu {
                *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            let k_step = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            let receipt: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            entries.push(Entry { kind, who, mu, k_step, receipt });
        }
        let mut escrowed = crate::network::nerve::Keyed::new();
        if let Some(c4) = b.get(*pos..*pos + 4) {
            let ne = u32::from_le_bytes(c4.try_into().ok()?) as usize;
            *pos += 4;
            if ne > b.len().saturating_sub(*pos) / 108 {
                return None; // untrusted count: ≥108 bytes per award
            }
            for _ in 0..ne {
                let addr = crate::transducer::a21::take_addr(b, pos)?;
                let nv = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
                *pos += 4;
                if nv > b.len().saturating_sub(*pos) / 32 {
                    return None;
                }
                let mut voices = Vec::with_capacity(nv);
                for _ in 0..nv {
                    voices.push(b.get(*pos..*pos + 32)?.try_into().ok()?);
                    *pos += 32;
                }
                let mut mu = [0u64; 4];
                for m in &mut mu {
                    *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let r: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                escrowed.insert(addr, (voices, mu, (k, r)));
            }
        }
        let mut t = Treasury { entries, bond_total, stakes, escrowed, balances: crate::network::nerve::Keyed::new() };
        t.refresh_bond();
        t.recompute_balances();
        Some(t)
    }

    fn recompute_balances(&mut self) {
        self.balances = crate::network::nerve::Keyed::new();
        let mut i = 0;
        while i < self.entries.len() {
            let who = self.entries[i].who;
            let mu = self.entries[i].mu;
            match self.entries[i].kind {
                "mint" => self.credit(who, mu),
                "transfer" => {
                    self.debit(who, mu);
                    if i + 1 < self.entries.len() {
                        let nxt = &self.entries[i + 1];
                        if nxt.kind == "transfer"
                            && nxt.mu == mu
                            && nxt.k_step == self.entries[i].k_step
                            && nxt.receipt == self.entries[i].receipt
                            && nxt.who != who
                        {
                            self.credit(nxt.who, nxt.mu);
                            i += 2;
                            continue;
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    fn germ() -> Element {
        // A germinated pattern (post-excision, the only species of germ).
        from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap())
    }

    #[test]
    fn novelty_pays_on_use_not_on_existence() {
        let mut t = Treasury::new();
        t.enroll([1; 32], UNIT_BOND, (0, [0; 32])); // genesis: the bond must exist
        let g = germ();
        let anchor: Anchor = (3, [7; 32]);
        assert!(magnitude(&g).iter().any(|&m| m >= 1), "a germinated pattern has cost on the lanes it moves");
        assert!(t.settle_install(&g, [1; 32]), "gate admits a reducing germ");
        assert_eq!(t.balance_mu(&[1; 32]), [0; 4], "installing pays NOTHING — no corpus proven yet");
        // CIRCULATION: a funded demander pays the discoverer on each real use.
        t.settle_subsidy([2; 32], [0, 0, 100, 0], anchor); // the demander earned earlier
        t.settle_use([2; 32], [1; 32], magnitude(&g), anchor); // a real demand decomposed through it
        t.settle_use([2; 32], [1; 32], magnitude(&g), anchor);
        let bal = t.balance_mu(&[1; 32]);
        assert_eq!(bal[2], 2 * magnitude(&g)[2], "reward flows with real use, per lane");
        assert_eq!(bal[0], 0, "no lane is invented");
        assert_eq!(t.balance_mu(&[2; 32])[2], 100 - 2 * magnitude(&g)[2], "the demander PAID — a transfer, not a mint");
        // Every YIELD entry carries its chronometric anchor — the unforgeable
        // binding (the genesis stake carries its own enrollment anchor).
        assert!(t.entries().iter().filter(|e| e.kind != "stake").all(|e| e.k_step == 3 && e.receipt == [7; 32]));
    }

    /// The dimensional ledger: consilience splits the PRICED μ⃗ per lane (nonzero
    /// lanes never split to zero), and the torse coin books on the holonomic lane.
    #[test]
    fn consilience_and_torse_settle_as_vectors() {
        let mut t = Treasury::new();
        t.enroll([1; 32], UNIT_BOND, (0, [0; 32])); // capitalize before settling
        let anchor: Anchor = (9, [1; 32]);
        t.settle_consilience(&[[1; 32], [2; 32]], [0, 0, 5, 0], anchor);
        assert_eq!(t.balance_mu(&[1; 32]), [0, 0, 2, 0]);
        assert_eq!(t.balance_mu(&[2; 32]), [0, 0, 2, 0]);
        t.settle_torse([1; 32], [0, 0, 40, 0], anchor);
        assert_eq!(
            t.balance_mu(&[1; 32])[2],
            42,
            "the torse coin mints the measured vector"
        );
        // The vector ledger round-trips with anchors intact.
        let snap = t.snapshot();
        let mut pos = 0;
        let back = Treasury::restore(&snap, &mut pos).expect("restore");
        assert_eq!(back.entries(), t.entries(), "μ⃗ entries + anchors survive, exactly");
    }

    /// A release with an empty stake bond still pays the measured vector.
    #[test]
    fn release_mints_when_the_bond_is_empty() {
        let mut t = Treasury::new();
        let addr = [7u8; 32];
        t.escrow_award(addr, vec![[1; 32], [2; 32]], [4, 0, 2, 0], (1, [9; 32]));
        assert_eq!(t.bond_total(), [0; 4]);
        let (voices, mu, anchor) = t.release_award(&addr).expect("claim");
        t.mint_claim(&voices, mu, anchor);
        assert_eq!(t.balance_mu(&[1; 32]), [2, 0, 1, 0]);
        assert_eq!(t.balance_mu(&[2; 32]), [2, 0, 1, 0]);
        assert_eq!(t.supply(), [4, 0, 2, 0]);
        assert_eq!(t.bond_total(), [0; 4], "mint does not touch the stake bond");
    }

    /// Supply after N installs equals the sum of their deflations per lane.
    #[test]
    fn supply_is_the_sum_of_minted_deflations() {
        let mut t = Treasury::new();
        t.enroll([1; 32], UNIT_BOND, (0, [0; 32]));
        let d1 = [4u64, 2, 0, 2];
        let d2 = [0u64, 4, 6, 2];
        t.escrow_award([8; 32], vec![[1; 32], [2; 32]], d1, (1, [1; 32]));
        t.escrow_award([9; 32], vec![[1; 32], [2; 32]], d2, (2, [2; 32]));
        let (v1, m1, a1) = t.release_award(&[8; 32]).expect("d1");
        t.mint_claim(&v1, m1, a1);
        let (v2, m2, a2) = t.release_award(&[9; 32]).expect("d2");
        t.mint_claim(&v2, m2, a2);
        let expect: [u64; 4] = std::array::from_fn(|l| d1[l] + d2[l]);
        assert_eq!(t.supply(), expect);
    }

    /// A slash debits the bond and mints nothing.
    #[test]
    fn slash_debits_the_bond_and_mints_nothing() {
        let mut t = Treasury::new();
        let bond = t.enroll([1; 32], [8, 8, 8, 8], (0, [0; 32]));
        let addr = [3u8; 32];
        t.escrow_award(addr, vec![[1; 32]], [8, 0, 0, 0], (1, [1; 32]));
        assert!(t.revoke_award(&addr, true));
        assert_eq!(t.bond_of(&[1; 32]).unwrap()[0], bond[0].saturating_sub(8));
        assert_eq!(t.supply(), [0; 4]);
        assert_eq!(t.balance_mu(&[1; 32]), [0; 4]);
    }

    /// Pool + stakes survive the v3 snapshot exactly.
    #[test]
    fn escrow_snapshot_round_trips() {
        let mut t = Treasury::new();
        t.enroll([3; 32], UNIT_BOND, (0, [4; 32]));
        t.settle_subsidy([3; 32], [0, 0, 5, 0], (1, [4; 32]));
        t.settle_recall([3; 32], [false, false, true, false], (2, [4; 32]));
        let snap = t.snapshot();
        let mut pos = 0;
        let back = Treasury::restore(&snap, &mut pos).expect("restore v3");
        assert_eq!(back.bond_total(), t.bond_total(), "live bonds survive, per lane");
        assert_eq!(back.stakes(), t.stakes(), "stakes survive");
        assert_eq!(back.entries(), t.entries());
    }

    #[test]
    fn the_dm_gate_is_per_lane() {
        assert!(descends(&[0, 0, 0, 0], &[1, 0, 0, 0]), "strict on one lane");
        assert!(!descends(&[1, 0, 0, 0], &[0, 1, 0, 0]), "lanes are not traded");
        assert!(!descends(&[1, 1, 0, 0], &[1, 1, 0, 0]), "equal is not descent");
    }


}
