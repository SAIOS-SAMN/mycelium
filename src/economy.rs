// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ECONOMY — THE DIMENSIONAL LEDGER (alpha Step 4). Every balance and every
//! receipt is a polytonic μ⃗ 4-vector (𝒮, 𝒢, residual, 𝒽) — scalar amounts are
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
/// (shift/rotation/scale), 𝒢 = relabeled-hue count, residual = changed-cell count,
/// 𝒽 reserved. A settled demand's residual magnitude is the zero vector; a real
/// germ's is nonzero — hence the descent.
pub fn magnitude(g: &Element) -> [u64; 4] {
    g.magnitude()
}

/// The per-lane Dershowitz–Manna descent: `after ≺_DM before` iff after ≤ before
/// on every lane and strictly less on some lane. No lane is traded for another.
pub fn descends(after: &[u64; 4], before: &[u64; 4]) -> bool {
    after.iter().zip(before).all(|(a, b)| a <= b) && after.iter().zip(before).any(|(a, b)| a < b)
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

/// Price a germ's work against the surface: `μ⃗[l] = magnitude[l] · (1 + surface[l])`
/// — work-size scaled by the lane's systemic resistance.
pub fn price(mag: &[u64; 4], surface: &[u64; 4]) -> [u64; 4] {
    std::array::from_fn(|l| mag[l] * (1 + surface[l]))
}

/// The chronometric anchor of a ledger entry: (K-step, chain head hash).
pub type Anchor = (u64, [u8; 32]);

/// Economic tiers for a decentralized mesh running cryptographic semantic
/// operations: yields scale by the participant's tier. A hypha/client's standing
/// weights its reward, so the topology supports staked, tiered roles — never a
/// flat one-class network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Founder,
    Validator,
    Participant,
    Observer,
}

impl Tier {
    pub fn weight(self) -> i64 {
        match self {
            Tier::Founder => 8,
            Tier::Validator => 4,
            Tier::Participant => 2,
            Tier::Observer => 1,
        }
    }
    pub fn from_str(s: &str) -> Tier {
        match s {
            "founder" => Tier::Founder,
            "validator" => Tier::Validator,
            "observer" => Tier::Observer,
            _ => Tier::Participant,
        }
    }
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
/// (markers whose μ⃗ moved escrow, not a balance: the stake capitalized the
/// pool; finality is a seal). An exhaustive total function — an unknown kind
/// orients nothing rather than silently crediting.
pub fn orientation(kind: &str) -> i8 {
    match kind {
        "utility" | "consilience" | "recall" | "torse" => 1,
        "spend" | "fee" | "slash" => -1,
        _ => 0, // "stake", "finality", forward-compatible unknowns
    }
}

/// The per-lane stake base a tier multiplies — the residual lane runs deeper
/// (that is where the work lives). Genesis issuance happens ONCE per identity,
/// at enrollment; thereafter the system conserves: pool + balances = Σ stakes.
pub const STAKE_BASE: [u64; 4] = [8, 8, 32, 8];

#[derive(Debug, Default, Clone)]
pub struct Treasury {
    entries: Vec<Entry>,
    /// THE ESCROW POOL — the finite per-lane reserve every yield is PAID FROM.
    /// Nothing is minted by a formula: prices are relative weights; the pool is
    /// the absolute scale (P5 — dynamic weights anchored to reality).
    escrow: [u64; 4],
    /// Enrolled identities and their staked μ⃗ (Vec-of-relations).
    stakes: Vec<([u8; 32], Tier, [u64; 4])>,
    /// AWARDS IN ESCROW (Step 5): consilience settlements held unfinalized —
    /// (germ address, attesting voices, μ⃗, anchor). Only the court's verdict
    /// releases (pays) or revokes (drops / slashes) them. Provisional installs
    /// earn nothing until linguistic finality.
    escrowed: Vec<([u8; 32], Vec<[u8; 32]>, [u64; 4], Anchor)>,
}

impl Treasury {
    pub fn new() -> Self {
        Treasury { entries: Vec::new(), escrow: [0; 4], stakes: Vec::new(), escrowed: Vec::new() }
    }

    /// ENROLL an identity: the stake (tier × STAKE_BASE, per lane) is issued ONCE
    /// and capitalizes the escrow pool — the genesis event; conservation holds
    /// thereafter. Idempotent per identity (one stake admits, C9). Returns the
    /// staked vector ([0;4] if already enrolled).
    pub fn enroll(&mut self, who: [u8; 32], tier: Tier, anchor: Anchor) -> [u64; 4] {
        if self.stakes.iter().any(|(w, _, _)| *w == who) {
            return [0; 4];
        }
        let w = tier.weight() as u64;
        let stake: [u64; 4] = std::array::from_fn(|l| STAKE_BASE[l] * w);
        for l in 0..4 {
            self.escrow[l] = self.escrow[l].saturating_add(stake[l]);
        }
        self.stakes.push((who, tier, stake));
        self.entries.push(Entry { kind: "stake", who, mu: stake, k_step: anchor.0, receipt: anchor.1 });
        stake
    }

    /// The escrow pool — what the market can actually pay, per lane.
    pub fn pool(&self) -> [u64; 4] {
        self.escrow
    }

    /// HOLD a consilience award in escrow pending the court's finality verdict
    /// (Step 5: two-voice consilience is provisional, not final). Nothing books
    /// to the ledger yet — unfinalized speech earns nothing.
    pub fn escrow_award(&mut self, addr: [u8; 32], voices: Vec<[u8; 32]>, mu: [u64; 4], anchor: Anchor) {
        if self.escrowed.iter().any(|(a, _, _, _)| *a == addr) {
            return; // one docket per germ
        }
        self.escrowed.push((addr, voices, mu, anchor));
    }

    pub fn escrowed_awards(&self) -> &[([u8; 32], Vec<[u8; 32]>, [u64; 4], Anchor)] {
        &self.escrowed
    }

    /// The court UPHELD the germ: release the escrowed award — the consilience
    /// split pays now (float-law bounded), and a zero-μ "finality" marker entry
    /// binds the ruling to the chain. Returns true if an award was released.
    pub fn release_award(&mut self, addr: &[u8; 32]) -> bool {
        let Some(i) = self.escrowed.iter().position(|(a, _, _, _)| a == addr) else {
            return false;
        };
        let (_, voices, mu, anchor) = self.escrowed.remove(i);
        self.entries.push(Entry { kind: "finality", who: *addr, mu: [0; 4], k_step: anchor.0, receipt: anchor.1 });
        self.settle_consilience(&voices, mu, anchor);
        true
    }

    /// The court REJECTED the germ: the escrowed award evaporates. With `slash`
    /// (a failed blind replay — false speech, not mere non-confluence) each
    /// voice is debited its would-be share from earned balance INTO the pool.
    pub fn revoke_award(&mut self, addr: &[u8; 32], slash: bool) -> bool {
        let Some(i) = self.escrowed.iter().position(|(a, _, _, _)| a == addr) else {
            return false;
        };
        let (_, voices, mu, anchor) = self.escrowed.remove(i);
        if slash {
            let n = voices.len().max(1) as u64;
            for v in &voices {
                let held = self.balance_mu(v);
                let cut: [u64; 4] = std::array::from_fn(|l| (mu[l] / n).min(held[l]));
                if cut.iter().any(|&c| c > 0) {
                    for l in 0..4 {
                        self.escrow[l] = self.escrow[l].saturating_add(cut[l]);
                    }
                    self.entries.push(Entry { kind: "slash", who: *v, mu: cut, k_step: anchor.0, receipt: anchor.1 });
                }
            }
        }
        true
    }

    pub fn stakes(&self) -> &[([u8; 32], Tier, [u64; 4])] {
        &self.stakes
    }

    /// PAY from the pool — the conservation law: each lane pays min(asked,
    /// escrow); the pool debits what the earner credits; an empty pool pays
    /// nothing and the ledger records only what was REAL. Returns the paid μ⃗.
    fn pay(&mut self, kind: &'static str, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let paid: [u64; 4] = std::array::from_fn(|l| mu[l].min(self.escrow[l]));
        if paid.iter().all(|&m| m == 0) {
            return paid;
        }
        for l in 0..4 {
            self.escrow[l] -= paid[l];
        }
        self.entries.push(Entry { kind, who, mu: paid, k_step: anchor.0, receipt: anchor.1 });
        paid
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
    /// Of every use-payment, the discoverer keeps 7/8 and 1/8 TITHES to the
    /// pool — the market-maker's cut that refills the subsidy float in
    /// proportion to real consumption, so discovery bounties never die at an
    /// empty pool while value still circulates (the spend entry carries the
    /// full amount; spend − utility on the same anchor IS the audited tithe).
    pub fn settle_use(&mut self, demander: [u8; 32], discoverer: [u8; 32], mag: [u64; 4], anchor: Anchor) -> [u64; 4] {
        if demander == discoverer {
            return [0; 4];
        }
        let held = self.balance_mu(&demander);
        let paid: [u64; 4] = std::array::from_fn(|l| mag[l].min(held[l]));
        if paid.iter().all(|&m| m == 0) {
            return paid;
        }
        let tithe: [u64; 4] = std::array::from_fn(|l| paid[l] / 8);
        for l in 0..4 {
            self.escrow[l] = self.escrow[l].saturating_add(tithe[l]);
        }
        self.entries.push(Entry { kind: "spend", who: demander, mu: paid, k_step: anchor.0, receipt: anchor.1 });
        let kept: [u64; 4] = std::array::from_fn(|l| paid[l] - tithe[l]);
        self.entries.push(Entry { kind: "utility", who: discoverer, mu: kept, k_step: anchor.0, receipt: anchor.1 });
        paid
    }

    /// A SUBSIDY from the pool (system-initiated yields with no payer: the
    /// redundancy probe, bootstrap awards). Bounded by the pool, like all pay.
    pub fn settle_subsidy(&mut self, who: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        self.pay("utility", who, mu, anchor)
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
            let bounded = self.award_bound(share);
            self.pay("consilience", *v, bounded, anchor);
        }
    }

    /// THE FLOAT LAW: a single award never takes more than pool/16 per lane
    /// (floor 1 while the lane holds anything). The formula RANKS; the pool
    /// SCALES — awards decay geometrically with the float instead of zeroing
    /// it, so the price signal breathes: rich pool → rich bounties, poor pool
    /// → poor ones, and tithe/fee inflow lifts both.
    fn award_bound(&self, asked: [u64; 4]) -> [u64; 4] {
        std::array::from_fn(|l| {
            let cap = (self.escrow[l] / 16).max((self.escrow[l] > 0) as u64);
            asked[l].min(cap)
        })
    }

    /// The TORSE coin — structure condensed from disorder: the strongest yield,
    /// paid on the residual lane to the bundle's discoverer, ∝ sections spanned
    /// priced at the surface.
    pub fn settle_torse(&mut self, discoverer: [u8; 32], mu: [u64; 4], anchor: Anchor) -> [u64; 4] {
        let bounded = self.award_bound(mu);
        self.pay("torse", discoverer, bounded, anchor)
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
        for l in 0..4 {
            self.escrow[l] = self.escrow[l].saturating_add(fee[l]);
        }
        self.entries.push(Entry { kind: "fee", who: voice, mu: fee, k_step: anchor.0, receipt: anchor.1 });
        fee
    }

    /// The POLYTONIC BALANCE — per-lane orientation-weighted sums; there is no
    /// scalar balance. Conservation reads pool + balances = Σ stakes, per lane.
    pub fn balance_mu(&self, who: &[u8; 32]) -> [u64; 4] {
        let mut out = [0u64; 4];
        for e in self.entries.iter().filter(|e| e.who == *who) {
            match orientation(e.kind) {
                1 => (0..4).for_each(|l| out[l] = out[l].saturating_add(e.mu[l])),
                -1 => (0..4).for_each(|l| out[l] = out[l].saturating_sub(e.mu[l])),
                _ => {} // 0-oriented: markers and stakes touch no balance
            }
        }
        out
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// Serialize the dimensional ledger: `[n][tag u8][who 32][μ⃗ 4×u64][k u64]
    /// [receipt 32]…` — vector amounts and chronometric anchors survive restart.
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for p in &self.escrow {
            out.extend_from_slice(&p.to_le_bytes());
        }
        out.extend_from_slice(&(self.stakes.len() as u32).to_le_bytes());
        for (who, tier, mu) in &self.stakes {
            out.extend_from_slice(who);
            out.push(tier.weight() as u8);
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
        }
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {
            let tag: u8 = match e.kind {
                "utility" => 0,
                "consilience" => 1,
                "recall" => 2,
                "torse" => 3,
                "stake" => 4,
                "spend" => 5,
                "fee" => 6,
                "finality" => 7,
                "slash" => 8,
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
        for (addr, voices, mu, anchor) in &self.escrowed {
            out.extend_from_slice(addr);
            out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
            for v in voices {
                out.extend_from_slice(v);
            }
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
            out.extend_from_slice(&anchor.0.to_le_bytes());
            out.extend_from_slice(&anchor.1);
        }
        out
    }

    /// Reload a ledger written by [`Treasury::snapshot`]. `None` on corruption.
    pub fn restore(b: &[u8], pos: &mut usize) -> Option<Treasury> {
        let mut escrow = [0u64; 4];
        for p in &mut escrow {
            *p = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let ns = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        if ns > b.len().saturating_sub(*pos) / 65 {
            return None; // untrusted count: 65 bytes per stake
        }
        let mut stakes = Vec::with_capacity(ns);
        for _ in 0..ns {
            let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            let w = *b.get(*pos)?;
            *pos += 1;
            let tier = match w {
                8 => Tier::Founder,
                4 => Tier::Validator,
                1 => Tier::Observer,
                _ => Tier::Participant,
            };
            let mut mu = [0u64; 4];
            for m in &mut mu {
                *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            stakes.push((who, tier, mu));
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
                0 => "utility",
                1 => "consilience",
                2 => "recall",
                3 => "torse",
                4 => "stake",
                5 => "spend",
                6 => "fee",
                7 => "finality",
                8 => "slash",
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
        let mut escrowed = Vec::new();
        if let Some(c4) = b.get(*pos..*pos + 4) {
            let ne = u32::from_le_bytes(c4.try_into().ok()?) as usize;
            *pos += 4;
            if ne > b.len().saturating_sub(*pos) / 108 {
                return None; // untrusted count: ≥108 bytes per award
            }
            for _ in 0..ne {
                let addr: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
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
                escrowed.push((addr, voices, mu, (k, r)));
            }
        }
        Some(Treasury { entries, escrow, stakes, escrowed })
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
        t.enroll([1; 32], Tier::Founder, (0, [0; 32])); // genesis: the pool must exist to pay
        let g = germ();
        let anchor: Anchor = (3, [7; 32]);
        assert!(magnitude(&g)[2] >= 1, "a germinated pattern has residual-lane cost");
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
    /// lanes never split to zero), and the torse coin books on the residual lane.
    #[test]
    fn consilience_and_torse_settle_as_vectors() {
        let mut t = Treasury::new();
        t.enroll([1; 32], Tier::Founder, (0, [0; 32])); // capitalize before settling
        let anchor: Anchor = (9, [1; 32]);
        t.settle_consilience(&[[1; 32], [2; 32]], [0, 0, 5, 0], anchor);
        assert_eq!(t.balance_mu(&[1; 32]), [0, 0, 2, 0]);
        assert_eq!(t.balance_mu(&[2; 32]), [0, 0, 2, 0]);
        // The FLOAT LAW bounds the torse coin to pool/16 on the lane.
        let cap = t.pool()[2] / 16;
        t.settle_torse([1; 32], [0, 0, 40, 0], anchor);
        assert_eq!(
            t.balance_mu(&[1; 32])[2],
            2 + 40u64.min(cap),
            "the torse coin lands on the lane, bounded by the float law"
        );
        // The vector ledger round-trips with anchors intact.
        let snap = t.snapshot();
        let mut pos = 0;
        let back = Treasury::restore(&snap, &mut pos).expect("restore");
        assert_eq!(back.entries(), t.entries(), "μ⃗ entries + anchors survive, exactly");
    }

    /// P5 — THE CONSERVATION LAW: pool + balances = Σ stakes, on every lane,
    /// after any settlement sequence. Nothing is minted by a formula.
    #[test]
    fn escrow_conserves_per_lane() {
        let mut t = Treasury::new();
        let s1 = t.enroll([1; 32], Tier::Founder, (0, [0; 32]));
        let s2 = t.enroll([2; 32], Tier::Participant, (0, [0; 32]));
        let supply: [u64; 4] = std::array::from_fn(|l| s1[l] + s2[l]);
        assert_eq!(t.pool(), supply, "stakes capitalize the pool exactly");
        assert_eq!(t.enroll([1; 32], Tier::Founder, (0, [0; 32])), [0; 4], "one stake admits — idempotent");
        t.settle_consilience(&[[1; 32], [2; 32]], [0, 0, 9, 0], (2, [1; 32]));
        t.settle_use([1; 32], [2; 32], [0, 0, 3, 0], (1, [1; 32])); // peer transfer
        t.settle_recall([2; 32], [false, false, true, false], (2, [1; 32])); // fee → pool
        t.settle_torse([2; 32], [0, 0, 40, 0], (3, [1; 32]));
        let held: [u64; 4] =
            std::array::from_fn(|l| t.balance_mu(&[1; 32])[l] + t.balance_mu(&[2; 32])[l]);
        for l in 0..4 {
            assert_eq!(t.pool()[l] + held[l], supply[l], "lane {l}: pool + balances = Σ stakes");
        }
    }

    /// An exhausted pool pays partial then nothing — the ledger records only
    /// what was REAL; the astronomical formula-price can never be booked.
    #[test]
    fn empty_pool_pays_nothing() {
        let mut t = Treasury::new();
        t.enroll([1; 32], Tier::Observer, (0, [0; 32])); // smallest stake
        let supply = t.pool();
        // Ask for astronomically more than exists — pays exactly the pool, once.
        let paid = t.settle_subsidy([1; 32], [u64::MAX; 4], (1, [1; 32]));
        assert_eq!(paid, supply, "pays the whole pool, never more");
        assert_eq!(t.pool(), [0; 4]);
        let again = t.settle_subsidy([1; 32], [0, 0, 694154, 0], (2, [1; 32]));
        assert_eq!(again, [0; 4], "an empty pool books NOTHING — honest scarcity");
        assert_eq!(t.balance_mu(&[1; 32]), supply, "total earned = total staked; conservation");
        // A broke demander transfers nothing; the discoverer earns only what was real.
        let broke = t.settle_use([9; 32], [1; 32], [0, 0, 50, 0], (3, [1; 32]));
        assert_eq!(broke, [0; 4], "no balance, no transfer — never a mint");
    }

    /// THE POOL BREATHES: discovery subsidies drain it, consumption fees refill
    /// it — the market's absolute scale tracks real circulation, never dying at
    /// zero while value still moves.
    #[test]
    fn fees_recharge_the_pool() {
        let mut t = Treasury::new();
        t.enroll([1; 32], Tier::Participant, (0, [0; 32]));
        t.settle_subsidy([1; 32], [0, 0, 10, 0], (1, [2; 32])); // pool → earner
        let drained = t.pool();
        let fee = t.settle_recall([1; 32], [false, false, true, false], (2, [2; 32]));
        assert_eq!(fee, [0, 0, 1, 0], "the reader paid the lane's unit fee");
        assert_eq!(t.pool()[2], drained[2] + 1, "the fee flowed INTO the pool — recharge");
        assert_eq!(t.balance_mu(&[1; 32])[2], 9, "and OUT of the reader's balance");
        // Wash trade pays nothing: a demander citing their own word transfers zero.
        assert_eq!(t.settle_use([1; 32], [1; 32], [0, 0, 5, 0], (3, [2; 32])), [0; 4]);
        // THE TITHE: a large use-payment returns 1/8 to the pool — proportional
        // recharge; the discoverer keeps 7/8; conservation closes exactly.
        let before = t.pool();
        let paid = t.settle_use([1; 32], [3; 32], [0, 0, 8, 0], (4, [2; 32]));
        assert_eq!(paid, [0, 0, 8, 0]);
        assert_eq!(t.pool()[2], before[2] + 1, "1/8 tithed into the pool");
        assert_eq!(t.balance_mu(&[3; 32])[2], 7, "the discoverer kept 7/8");
    }

    /// Pool + stakes survive the v3 snapshot exactly.
    #[test]
    fn escrow_snapshot_round_trips() {
        let mut t = Treasury::new();
        t.enroll([3; 32], Tier::Validator, (0, [4; 32]));
        t.settle_subsidy([3; 32], [0, 0, 5, 0], (1, [4; 32]));
        t.settle_recall([3; 32], [false, false, true, false], (2, [4; 32]));
        let snap = t.snapshot();
        let mut pos = 0;
        let back = Treasury::restore(&snap, &mut pos).expect("restore v3");
        assert_eq!(back.pool(), t.pool(), "the escrow pool survives, per lane");
        assert_eq!(back.stakes(), t.stakes(), "stakes + tiers survive");
        assert_eq!(back.entries(), t.entries());
    }

    #[test]
    fn the_dm_gate_is_per_lane() {
        assert!(descends(&[0, 0, 0, 0], &[1, 0, 0, 0]), "strict on one lane");
        assert!(!descends(&[1, 0, 0, 0], &[0, 1, 0, 0]), "lanes are not traded");
        assert!(!descends(&[1, 1, 0, 0], &[1, 1, 0, 0]), "equal is not descent");
    }


}
