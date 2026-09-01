// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ECONOMY — the settlement ledger and POW++ pricing. A germ pays only when it
//! REDUCES: the demand's polytonic magnitude must descend under the per-lane
//! Dershowitz–Manna order (never a summed score). Bounties credit the voices
//! that germinated; consilience splits a fixed reward on the two-voice crossing.
//! The pricing decision (≺_DM) is topological and map-free; the ledger is a
//! below-boundary Vec of receipts.

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

const CONSILIENCE: i64 = 16;

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
/// The UTILITY yield paid to a germ's DISCOVERER each time a real later demand
/// recalls it — reward is proportional to corpus actually unlocked, never a flat
/// bounty for a germ that merely closes its own birth demand (anti-Goodhart).
const USE_YIELD: i64 = 5;

/// A settlement receipt (below-boundary storage — a Vec, not a map).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: &'static str,
    pub who: [u8; 32],
    pub amount: i64,
}

#[derive(Debug, Default, Clone)]
pub struct Treasury {
    entries: Vec<Entry>,
}

impl Treasury {
    pub fn new() -> Self {
        Treasury { entries: Vec::new() }
    }

    fn credit(&mut self, kind: &'static str, who: [u8; 32], amount: i64) {
        self.entries.push(Entry { kind, who, amount });
    }

    /// The POW++ GATE on a germ install: it admits iff the germ reduces the demand
    /// (`0⃗ ≺_DM magnitude`). NB: installing pays NOTHING here — a germ that only
    /// closes its own birth demand has proven nothing about the corpus. Reward
    /// flows later, on USE (`settle_use`), proportional to real demand unlocked.
    pub fn settle_install(&mut self, g: &Element, _voice: [u8; 32]) -> bool {
        descends(&[0u64; 4], &magnitude(g))
    }

    /// A germ was USED — a real later demand recalled it. Pay its DISCOVERER the
    /// utility yield: this is the corpus actually unlocked, the only novelty the
    /// economy prices. Called once per recall of an installed germ.
    pub fn settle_use(&mut self, discoverer: [u8; 32]) {
        self.credit("utility", discoverer, USE_YIELD);
    }

    /// Tiered utility yield: the discoverer's tier weights the reward — the
    /// economic-tier hook for a decentralized, staked mesh.
    pub fn settle_use_tiered(&mut self, discoverer: [u8; 32], tier: Tier) {
        self.credit("utility", discoverer, USE_YIELD * tier.weight());
    }

    /// Split the consilience reward across the two voices on the admission event.
    pub fn settle_consilience(&mut self, voices: &[[u8; 32]]) {
        if voices.len() < 2 {
            return;
        }
        let share = (CONSILIENCE / voices.len() as i64).max(1);
        for v in voices {
            self.credit("consilience", *v, share);
        }
    }

    /// A recall (recognised) pays a small proportional yield to the voice.
    pub fn settle_recall(&mut self, voice: [u8; 32]) {
        self.credit("recall", voice, 1);
    }

    pub fn balance(&self, who: &[u8; 32]) -> i64 {
        self.entries.iter().filter(|e| e.who == *who).map(|e| e.amount).sum()
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// Serialize the ledger for persistence (P7): `[n][kind_tag u8][who 32][amount
    /// i64]…`. Kinds map to a stable tag so earned balances survive restart.
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {
            let tag: u8 = match e.kind {
                "utility" => 0,
                "consilience" => 1,
                "recall" => 2,
                _ => 255,
            };
            out.push(tag);
            out.extend_from_slice(&e.who);
            out.extend_from_slice(&e.amount.to_le_bytes());
        }
        out
    }

    /// Reload a ledger written by [`Treasury::snapshot`]. `None` on corruption.
    pub fn restore(b: &[u8], pos: &mut usize) -> Option<Treasury> {
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        if n > b.len().saturating_sub(*pos) / 41 {
            return None; // untrusted count: 41 bytes per entry, refuse before allocating
        }
        let mut entries = Vec::with_capacity(n);
        for _ in 0..n {
            let tag = *b.get(*pos)?;
            *pos += 1;
            let kind = match tag {
                0 => "utility",
                1 => "consilience",
                2 => "recall",
                _ => "unknown",
            };
            let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            let amount = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            entries.push(Entry { kind, who, amount });
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
        assert!(magnitude(&g)[2] >= 1, "a germinated pattern has residual-lane cost");
        assert!(t.settle_install(&g, [1; 32]), "gate admits a reducing germ");
        assert_eq!(t.balance(&[1; 32]), 0, "installing pays NOTHING — no corpus proven yet");
        t.settle_use([1; 32]); // a real demand later recalled it
        t.settle_use([1; 32]);
        assert_eq!(t.balance(&[1; 32]), 10, "reward flows with real use (2 × 5)");
    }

    #[test]
    fn the_dm_gate_is_per_lane() {
        assert!(descends(&[0, 0, 0, 0], &[1, 0, 0, 0]), "strict on one lane");
        assert!(!descends(&[1, 0, 0, 0], &[0, 1, 0, 0]), "lanes are not traded");
        assert!(!descends(&[1, 1, 0, 0], &[1, 1, 0, 0]), "equal is not descent");
    }

    #[test]
    fn consilience_splits_the_reward() {
        let mut t = Treasury::new();
        t.settle_consilience(&[[1; 32], [2; 32]]);
        assert_eq!(t.balance(&[1; 32]), 8);
        assert_eq!(t.balance(&[2; 32]), 8);
    }
}
