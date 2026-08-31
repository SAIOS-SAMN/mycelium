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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_transform;
    use crate::transducer::transform::solve_transform;

    fn germ() -> Element {
        let t = solve_transform(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 3, 5u8), (0, 4, 5), (1, 3, 5)]).unwrap();
        from_transform(&t)
    }

    #[test]
    fn novelty_pays_on_use_not_on_existence() {
        let mut t = Treasury::new();
        let g = germ();
        assert!(magnitude(&g)[0] >= 1 && magnitude(&g)[1] == 1, "motion∘recolour has S+G cost");
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
