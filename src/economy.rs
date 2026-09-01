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

#[derive(Debug, Default, Clone)]
pub struct Treasury {
    entries: Vec<Entry>,
}

impl Treasury {
    pub fn new() -> Self {
        Treasury { entries: Vec::new() }
    }

    fn credit(&mut self, kind: &'static str, who: [u8; 32], mu: [u64; 4], anchor: Anchor) {
        // A zero vector books nothing — the ledger records work, not noise.
        if mu.iter().all(|&m| m == 0) {
            return;
        }
        self.entries.push(Entry { kind, who, mu, k_step: anchor.0, receipt: anchor.1 });
    }

    /// The POW++ GATE on a germ install: it admits iff the germ reduces the demand
    /// (`0⃗ ≺_DM magnitude`). NB: installing pays NOTHING here — a germ that only
    /// closes its own birth demand has proven nothing about the corpus. Reward
    /// flows later, on USE (`settle_use`), proportional to real demand unlocked.
    pub fn settle_install(&mut self, g: &Element, _voice: [u8; 32]) -> bool {
        descends(&[0u64; 4], &magnitude(g))
    }

    /// PoUW — a word was USED: a real later demand decomposed through it. Pay its
    /// DISCOVERER the word's per-lane structure exercised (μ⃗ = the magnitude) —
    /// the corpus actually unlocked, per lane, never a flat scalar bounty.
    pub fn settle_use(&mut self, discoverer: [u8; 32], mag: [u64; 4], anchor: Anchor) {
        self.credit("utility", discoverer, mag, anchor);
    }

    /// Tiered utility: the discoverer's stake tier weights the vector, per lane.
    pub fn settle_use_tiered(&mut self, discoverer: [u8; 32], mag: [u64; 4], tier: Tier, anchor: Anchor) {
        let w = tier.weight() as u64;
        self.credit("utility", discoverer, std::array::from_fn(|l| mag[l] * w), anchor);
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
            self.credit("consilience", *v, share, anchor);
        }
    }

    /// The TORSE coin — structure condensed from disorder: the strongest yield,
    /// paid on the residual lane to the bundle's discoverer, ∝ sections spanned
    /// priced at the surface.
    pub fn settle_torse(&mut self, discoverer: [u8; 32], mu: [u64; 4], anchor: Anchor) {
        self.credit("torse", discoverer, mu, anchor);
    }

    /// A reading (decomposition) pays the reader a unit on each lane the sentence
    /// exercised — proportional, per lane, small.
    pub fn settle_recall(&mut self, voice: [u8; 32], lanes: [bool; 4], anchor: Anchor) {
        self.credit("recall", voice, std::array::from_fn(|l| lanes[l] as u64), anchor);
    }

    /// The POLYTONIC BALANCE — per-lane sums; there is no scalar balance.
    pub fn balance_mu(&self, who: &[u8; 32]) -> [u64; 4] {
        let mut out = [0u64; 4];
        for e in self.entries.iter().filter(|e| e.who == *who) {
            for l in 0..4 {
                out[l] = out[l].saturating_add(e.mu[l]);
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
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {
            let tag: u8 = match e.kind {
                "utility" => 0,
                "consilience" => 1,
                "recall" => 2,
                "torse" => 3,
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
        out
    }

    /// Reload a ledger written by [`Treasury::snapshot`]. `None` on corruption.
    pub fn restore(b: &[u8], pos: &mut usize) -> Option<Treasury> {
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
        Some(Treasury { entries })
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
        let g = germ();
        let anchor: Anchor = (3, [7; 32]);
        assert!(magnitude(&g)[2] >= 1, "a germinated pattern has residual-lane cost");
        assert!(t.settle_install(&g, [1; 32]), "gate admits a reducing germ");
        assert_eq!(t.balance_mu(&[1; 32]), [0; 4], "installing pays NOTHING — no corpus proven yet");
        t.settle_use([1; 32], magnitude(&g), anchor); // a real demand decomposed through it
        t.settle_use([1; 32], magnitude(&g), anchor);
        let bal = t.balance_mu(&[1; 32]);
        assert_eq!(bal[2], 2 * magnitude(&g)[2], "reward flows with real use, per lane");
        assert_eq!(bal[0], 0, "no lane is invented");
        // Every entry carries its chronometric anchor — the unforgeable binding.
        assert!(t.entries().iter().all(|e| e.k_step == 3 && e.receipt == [7; 32]));
    }

    /// The dimensional ledger: consilience splits the PRICED μ⃗ per lane (nonzero
    /// lanes never split to zero), and the torse coin books on the residual lane.
    #[test]
    fn consilience_and_torse_settle_as_vectors() {
        let mut t = Treasury::new();
        let anchor: Anchor = (9, [1; 32]);
        t.settle_consilience(&[[1; 32], [2; 32]], [0, 0, 5, 0], anchor);
        assert_eq!(t.balance_mu(&[1; 32]), [0, 0, 2, 0]);
        assert_eq!(t.balance_mu(&[2; 32]), [0, 0, 2, 0]);
        t.settle_torse([1; 32], [0, 0, 40, 0], anchor);
        assert_eq!(t.balance_mu(&[1; 32]), [0, 0, 42, 0], "the torse coin lands on the lane");
        // The vector ledger round-trips with anchors intact.
        let snap = t.snapshot();
        let mut pos = 0;
        let back = Treasury::restore(&snap, &mut pos).expect("restore");
        assert_eq!(back.entries(), t.entries(), "μ⃗ entries + anchors survive, exactly");
    }

    #[test]
    fn the_dm_gate_is_per_lane() {
        assert!(descends(&[0, 0, 0, 0], &[1, 0, 0, 0]), "strict on one lane");
        assert!(!descends(&[1, 0, 0, 0], &[0, 1, 0, 0]), "lanes are not traded");
        assert!(!descends(&[1, 1, 0, 0], &[1, 1, 0, 0]), "equal is not descent");
    }


}
