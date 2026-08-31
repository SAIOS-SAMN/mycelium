// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ECONOMY — the settlement ledger and POW++ pricing. A germ pays only when it
//! REDUCES: the demand's polytonic magnitude must descend under the per-lane
//! Dershowitz–Manna order (never a summed score). Bounties credit the voices
//! that germinated; consilience splits a fixed reward on the two-voice crossing.
//! The pricing decision (≺_DM) is topological and map-free; the ledger is a
//! below-boundary Vec of receipts.

use crate::transducer::motor::D4;
use crate::transducer::transform::Transform;

/// A germ's polytonic magnitude — per-lane cost, never summed. 𝒮 = spatial
/// complexity (shift + non-identity rotation + non-unit scale), 𝒢 = relabel
/// size, 𝒯/𝒽 reserved. A settled demand's residual magnitude is the zero
/// vector; the pre-settlement demand's is nonzero — hence the descent.
pub fn magnitude(g: &Transform) -> [u64; 4] {
    use num_traits::{One, Zero};
    let (t0, t1) = &g.motor.translation;
    let mut s = 0u64;
    if !t0.is_zero() || !t1.is_zero() {
        s += 1;
    }
    if g.motor.rotation != D4::E {
        s += 1;
    }
    if !g.motor.dilation.is_one() {
        s += 1;
    }
    [s, g.relabel.len() as u64, 0, 0]
}

/// The per-lane Dershowitz–Manna descent: `after ≺_DM before` iff after ≤ before
/// on every lane and strictly less on some lane. No lane is traded for another.
pub fn descends(after: &[u64; 4], before: &[u64; 4]) -> bool {
    after.iter().zip(before).all(|(a, b)| a <= b) && after.iter().zip(before).any(|(a, b)| a < b)
}

const BOUNTY: i64 = 5;
const CONSILIENCE: i64 = 16;

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

    /// Pay a germ install: only if it descends (POW++ gate). The germ reduces the
    /// demand to nothing, so `after = 0⃗ ≺_DM magnitude(germ)` — it always
    /// descends; the gate refuses a non-reducing "settlement".
    pub fn settle_install(&mut self, g: &Transform, voice: [u8; 32]) -> bool {
        let before = magnitude(g);
        let after = [0u64; 4];
        if !descends(&after, &before) {
            return false; // nothing reduced — no pay
        }
        self.credit("mint", voice, BOUNTY);
        true
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
    use crate::transducer::transform::solve_transform;

    fn germ() -> Transform {
        solve_transform(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 3, 5u8), (0, 4, 5), (1, 3, 5)]).unwrap()
    }

    #[test]
    fn a_reducing_germ_pays_its_voice() {
        let mut t = Treasury::new();
        let g = germ();
        assert!(magnitude(&g)[0] >= 1 && magnitude(&g)[1] == 1, "motion∘recolour has S+G cost");
        assert!(t.settle_install(&g, [1; 32]));
        assert_eq!(t.balance(&[1; 32]), 5);
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
