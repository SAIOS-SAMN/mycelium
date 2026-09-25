// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A17, T5 — THAT A TORSION CYCLE CLOSES: the circuit of `σ` round the fibre,
//! and the exact residual when it does not. A period is believed only where
//! the orbit returns to base (∮ dX = 0); an open orbit names the anholonomity
//! the label was filing over.

use crate::core::Q;
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

/// The closed-orbit integrity witness: either the orbit CLOSES (∮ dX = 0 — the
/// orbit returned to its start, trivial monodromy) or the residual step count
/// names the anholonomity (how far from closure the run ended).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HolonomicOrbitWitness {
    /// ∮_γ dX = 0: the orbit closed in `period` steps — valency 1/period.
    Closed { period: u64, valency: HolonomicValency },
    /// The orbit did not close within the span run — the open residual.
    Open { run: u64 },
}

/// Run a torsion cycle's generator σ round from the thought's base assignment and
/// WITNESS the orbit: closure at step t ⇒ `Closed { t, 1/t }` (∮ = 0, exactly);
/// domain exhaustion or no closure within `max` ⇒ `Open`.
///
/// The witness RUNS the orbit on purpose — it is the independent check of a
/// period read off the cycles (78.43(2)) — and its run is set by the period
/// it checks (`run_of`), never by a constant. Steps in the period's carrier,
/// `u64`; a closure past `i64` has no valency to write and is not claimed.
pub fn witness_cycle(thought: &Element, sigma: &[(u8, u8)], max: u64) -> HolonomicOrbitWitness {
    let base = assignment(thought.sub());
    let mut cur = base.clone();
    for step in 1..=max {
        match apply_hue_map(sigma, &cur) {
            Some(next) => {
                if next == base {
                    let Ok(den) = i64::try_from(step) else {
                        return HolonomicOrbitWitness::Open { run: step - 1 };
                    };
                    return HolonomicOrbitWitness::Closed {
                        period: step,
                        valency: HolonomicValency::new(1, den),
                    };
                }
                cur = next;
            }
            None => return HolonomicOrbitWitness::Open { run: step - 1 },
        }
    }
    HolonomicOrbitWitness::Open { run: max }
}

/// Witness every torsion cycle a thought holds — the bundle's monodromy record.
/// How far a witness runs round a cycle of period ℓ: twice round, and at least
/// four steps so a period of one or two is still seen to close. A period
/// whose double does not fit the carrier is run as far as the carrier
/// counts, which is further than any orbit this reader holds.
fn run_of(period: u64) -> u64 {
    match period.checked_mul(2) {
        Some(twice) => twice.max(4),
        None => u64::MAX,
    }
}

/// ONE CYCLE WITNESSED BY ITS GENERATOR (PD1): a HUE cycle's orbit is walked
/// to twice its READ period — never the stored one, which is a claim — and a
/// HOLONOMY cycle is checked by the algebra, `ray^ℓ = 1` exactly, the ray's
/// own order, with no walk. A generator with no period read is not walked
/// at all: an open orbit of run zero.
fn witness_generator(thought: &Element, c: &crate::transducer::element::TorsionCycle) -> HolonomicOrbitWitness {
    use crate::transducer::element::Generator;
    match &c.generator {
        Generator::Hue(sigma) => match crate::transducer::element::orbit_period(sigma, &assignment(thought.sub())) {
            Ok(read) => witness_cycle(thought, sigma, run_of(read)),
            Err(_) => HolonomicOrbitWitness::Open { run: 0 },
        },
        Generator::Holonomy(_) => match c.generator.order().map(|l| (l, i64::try_from(l))) {
            Some((l, Ok(den))) => HolonomicOrbitWitness::Closed { period: l, valency: HolonomicValency::new(1, den) },
            _ => HolonomicOrbitWitness::Open { run: 0 },
        },
    }
}

pub fn witness_thought(thought: &Element) -> Vec<HolonomicOrbitWitness> {
    thought.torsion().iter().map(|c| witness_generator(thought, c)).collect()
}

/// Every stored cycle held against its witness: (the period ℓ the word
/// speaks, the orbit as run). A caller matches the witness arms: a
/// Closed orbit at the stored period is a witnessed winding; Closed at
/// another period is a misspoken winding; Open names the anholonomity the
/// label was filing over.
pub fn witness_stored(thought: &Element) -> Vec<(u64, HolonomicOrbitWitness)> {
    thought
        .torsion()
        .iter()
        .map(|c| (c.period, witness_generator(thought, c)))
        .collect()
}

/// The total winding of a thought — Σ valencies of its CLOSED orbits (exact ℚ;
/// an open orbit contributes nothing: only proven monodromy counts).
pub fn total_winding(witnesses: &[HolonomicOrbitWitness]) -> Q {
    witnesses.iter().fold(Q::zero(), |s, w| match w {
        HolonomicOrbitWitness::Closed { valency, .. } => s + valency.0.clone(),
        HolonomicOrbitWitness::Open { .. } => s,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_canonical;
    use crate::transducer::germinate::germinate;

    /// A torse-extracted ℤ/3 cycle carries a CLOSED orbit witness of period 3 and
    /// valency 1/3 — ∮ dX = 0 exactly; an unrelated open family witnesses Open.
    #[test]
    fn the_orbit_witness_closes_on_the_cycle_and_names_the_open() {
        let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
        let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
        let mut t = from_canonical(&germinate(&a, &b).unwrap());
        t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
        assert_eq!(t.torse(), 1);
        let ws = witness_thought(&t);
        assert_eq!(
            ws,
            vec![HolonomicOrbitWitness::Closed { period: 3, valency: HolonomicValency::new(1, 3) }],
            "the cycle's monodromy is proven, not assumed"
        );
        assert_eq!(total_winding(&ws), HolonomicValency::new(1, 3).0, "Σ windings, exact ℚ");
        // An open orbit (σ leaves its domain) witnesses Open with the residual span.
        let open = witness_cycle(&t, &[(1, 9)], 8);
        assert!(matches!(open, HolonomicOrbitWitness::Open { .. }), "no closure is never faked");
    }
}
