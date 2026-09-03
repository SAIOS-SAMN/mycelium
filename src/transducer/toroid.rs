// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! TOROIDAL CONSTRUCTS — ported from ipsal-autic `engine/toroidal_construct.rs`:
//! bounded toroidal windings as exact-ℚ HOLONOMIC VALENCIES (coprime n/d), and
//! the closed-loop integrity witness (Theorems XXXII/XXXIV): ∮_γ dX = 0 over a
//! walked trajectory — zero monodromy iff the loop properly closes; a non-zero
//! residual NAMES the torsion class. Bound here to the germinated axis: a
//! torsion cycle (σ, ℓ) is a winding of valency 1/ℓ whose loop witness is the
//! exact σ-walk returning to base.

use crate::core::engine::Q;
use crate::transducer::element::{apply_hue_map, assignment, Element};
use num_bigint::BigInt;
use num_traits::Zero;

/// A fibre winding as an exact rational valency (kept coprime by ℚ reduction).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HolonomicValency(pub Q);

impl HolonomicValency {
    pub fn new(n: i64, d: i64) -> Self {
        HolonomicValency(Q::new(BigInt::from(n), BigInt::from(d)))
    }
}

/// The closed-loop integrity witness: either the loop CLOSES (∮ dX = 0 — the
/// walk returned to its start, trivial monodromy) or the residual step count
/// names the anholonomity (how far from closure the walk ended).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HolonomicLoopWitness {
    /// ∮_γ dX = 0: the walk closed in `period` steps — valency 1/period.
    Closed { period: u32, valency: HolonomicValency },
    /// The loop did not close within the walked span — the open residual.
    Open { walked: u32 },
}

/// Walk a torsion cycle's generator σ from the thought's base assignment and
/// WITNESS the loop: closure at step t ⇒ `Closed { t, 1/t }` (∮ = 0, exactly);
/// domain exhaustion or no closure within `max` ⇒ `Open`.
pub fn witness_cycle(thought: &Element, sigma: &[(u8, u8)], max: u32) -> HolonomicLoopWitness {
    let base = assignment(thought.sub());
    let mut cur = base.clone();
    for step in 1..=max {
        match apply_hue_map(sigma, &cur) {
            Some(next) => {
                if next == base {
                    return HolonomicLoopWitness::Closed {
                        period: step,
                        valency: HolonomicValency::new(1, step as i64),
                    };
                }
                cur = next;
            }
            None => return HolonomicLoopWitness::Open { walked: step - 1 },
        }
    }
    HolonomicLoopWitness::Open { walked: max }
}

/// Witness every torsion cycle a thought holds — the bundle's monodromy record.
pub fn witness_thought(thought: &Element) -> Vec<HolonomicLoopWitness> {
    thought
        .torsion()
        .iter()
        .map(|(sigma, l)| witness_cycle(thought, sigma, l.saturating_mul(2).max(4)))
        .collect()
}

/// Every stored cycle held against its witness: (the period ℓ the word
/// speaks, the loop as walked). A caller matches the witness arms: a
/// Closed loop at the stored period is a witnessed winding; Closed at
/// another period is a misspoken winding; Open names the anholonomity the
/// label was filing over.
pub fn witness_stored(thought: &Element) -> Vec<(u32, HolonomicLoopWitness)> {
    thought
        .torsion()
        .iter()
        .map(|(sigma, l)| (*l, witness_cycle(thought, sigma, l.saturating_mul(2).max(4))))
        .collect()
}

/// The total winding of a thought — Σ valencies of its CLOSED loops (exact ℚ;
/// an open loop contributes nothing: only proven monodromy counts).
pub fn total_winding(witnesses: &[HolonomicLoopWitness]) -> Q {
    witnesses.iter().fold(Q::zero(), |s, w| match w {
        HolonomicLoopWitness::Closed { valency, .. } => s + valency.0.clone(),
        HolonomicLoopWitness::Open { .. } => s,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    /// A torse-extracted ℤ/3 cycle carries a CLOSED loop witness of period 3 and
    /// valency 1/3 — ∮ dX = 0 exactly; an unrelated open family witnesses Open.
    #[test]
    fn the_loop_witness_closes_on_the_cycle_and_names_the_open() {
        let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
        let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
        let mut t = from_residual(&germinate(&a, &b).unwrap());
        t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
        assert_eq!(t.torse(), 1);
        let ws = witness_thought(&t);
        assert_eq!(
            ws,
            vec![HolonomicLoopWitness::Closed { period: 3, valency: HolonomicValency::new(1, 3) }],
            "the cycle's monodromy is proven, not assumed"
        );
        assert_eq!(total_winding(&ws), HolonomicValency::new(1, 3).0, "Σ windings, exact ℚ");
        // An open walk (σ leaves its domain) witnesses Open with the residual span.
        let open = witness_cycle(&t, &[(1, 9)], 8);
        assert!(matches!(open, HolonomicLoopWitness::Open { .. }), "no closure is never faked");
    }
}
