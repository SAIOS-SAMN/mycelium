// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COHESION — ported from ipsal-autic `cognition/cohesion.rs` + CE's
//! DIMENSIONAL_RESOLUTION: cohesion is the JOINT fixed point (C, I) → (0, 0):
//!   C — coherence: closure quality (what fails to close — the open, un-settled
//!       demand the mesh holds);
//!   I — coverage: the vocabulary-reach distance (accumulated disorder the held
//!       language has not yet condensed — how far words fall short of the world).
//! The witness is a typed 4-arm on (C zero?, I zero?) with the κ/16 tolerance
//! band; `Cohesive` requires BOTH axes zero — the axes are never traded or
//! summed. Expansion (unfold new dimensional space) and collapse (project to
//! the irreducible core) are both bounded by the H¹ floor — the substrate can
//! never fold past its irreducible form. The pressure gradient feeds the yield
//! surface: a seam prices high on BOTH axes.

use crate::core::engine::Q;
use num_bigint::BigInt;
use num_traits::Zero;

fn q(v: u64) -> Q {
    Q::from(BigInt::from(v))
}

/// Which axis is at its fixed point while the other is not (the degenerate
/// states carry substantive content — LAW-0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsymmetricDirection {
    /// C = 0, I > 0: everything closes, but the vocabulary has not reached —
    /// the mesh should EXPAND (unfold coverage).
    CoherentButUncovered,
    /// C > 0, I = 0: the vocabulary reaches, but closure fails — the mesh
    /// should COLLAPSE (project toward the irreducible core).
    CoveredButIncoherent,
}

/// The cohesion witness — typed dispatch on the joint fixed point, tolerance
/// band κ/16 per axis, both axes carried exactly (never a summed scalar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CohesionWitness {
    /// (0, 0) exactly — the substrate's target.
    Cohesive,
    /// Both axes within the tolerance band (slack per axis carried).
    WithinTolerance { c: Q, i: Q, slack_c: Q, slack_i: Q },
    /// One axis at its fixed point, the other not — degenerate, directed.
    Asymmetric { c: Q, i: Q, direction: AsymmetricDirection },
    /// Both above tolerance — AT THE SEAM: the substrate is pursuing; the
    /// pressure (per-axis pair, not a sum) prices the seam on the surface.
    AtSeam { c: Q, i: Q },
}

/// The substrate-canonical tolerance: κ/16 with κ = 16 ⇒ slack 1 per axis.
const KAPPA: u64 = 16;

/// Witness the joint state from the two measured axes.
pub fn witness(c: Q, i: Q) -> CohesionWitness {
    let tol = q(KAPPA) / q(16);
    match (c.is_zero(), i.is_zero()) {
        (true, true) => CohesionWitness::Cohesive,
        (true, false) => {
            CohesionWitness::Asymmetric { c, i, direction: AsymmetricDirection::CoherentButUncovered }
        }
        (false, true) => {
            CohesionWitness::Asymmetric { c, i, direction: AsymmetricDirection::CoveredButIncoherent }
        }
        (false, false) => {
            if c <= tol && i <= tol {
                let slack_c = tol.clone() - c.clone();
                let slack_i = tol - i.clone();
                CohesionWitness::WithinTolerance { c, i, slack_c, slack_i }
            } else {
                CohesionWitness::AtSeam { c, i }
            }
        }
    }
}

/// The cohesion PRESSURE the witness exerts on the yield surface — per axis,
/// never summed: (C-pressure, I-pressure). Cohesive/WithinTolerance exert none;
/// an asymmetric state presses only its failing axis; the seam presses both.
pub fn pressure(w: &CohesionWitness) -> (Q, Q) {
    match w {
        CohesionWitness::Cohesive | CohesionWitness::WithinTolerance { .. } => (Q::zero(), Q::zero()),
        CohesionWitness::Asymmetric { c, i, .. } => (c.clone(), i.clone()),
        CohesionWitness::AtSeam { c, i } => (c.clone(), i.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The witness dispatches on the JOINT fixed point: both zero = cohesive;
    /// one zero = directed asymmetry; both small = tolerance with exact slack;
    /// both large = the seam. Axes are never traded.
    #[test]
    fn the_witness_holds_both_axes_and_never_trades_them() {
        assert_eq!(witness(q(0), q(0)), CohesionWitness::Cohesive);
        assert!(matches!(
            witness(q(0), q(7)),
            CohesionWitness::Asymmetric { direction: AsymmetricDirection::CoherentButUncovered, .. }
        ));
        assert!(matches!(
            witness(q(7), q(0)),
            CohesionWitness::Asymmetric { direction: AsymmetricDirection::CoveredButIncoherent, .. }
        ));
        match witness(q(1), q(1)) {
            CohesionWitness::WithinTolerance { slack_c, slack_i, .. } => {
                assert!(slack_c.is_zero() && slack_i.is_zero(), "exact slack at the band edge");
            }
            w => panic!("expected tolerance, got {w:?}"),
        }
        let seam = witness(q(9), q(5));
        assert!(matches!(seam, CohesionWitness::AtSeam { .. }));
        let (pc, pi) = pressure(&seam);
        assert_eq!((pc, pi), (q(9), q(5)), "seam pressure is the exact per-axis pair — no sum");
    }
}
